<script lang="ts">
  import { endpointStore } from '../../stores/endpoints.svelte.js';
  import { invoke } from '@tauri-apps/api/core';
  import MethodSelector from './MethodSelector.svelte';
  
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
  import { Select, SelectContent, SelectItem, SelectTrigger } from '$lib/components/ui/select';
  
  import { Plus, Trash2, GripVertical, ExternalLink } from '@lucide/svelte';
  import CodeMirror from '$lib/components/editor/CodeMirror.svelte';

  // Runes-derived app state
  let selectedEndpoint = $derived(endpointStore.selectedEndpoint);
  let requestConfig = $derived(endpointStore.requestConfig);
  let currentEnvironment = $derived(endpointStore.currentEnvironment);
  let isLoading = $derived(endpointStore.isLoading);
  let error = $derived(endpointStore.error);
  let activeTab = $state<'parameters' | 'body' | 'headers' | 'authorization' | 'prerequest' | 'postrequest' | 'variables'>('parameters');

  // Body content type (state)
  type BodyType = 'application/json' | 'text/plain' | 'application/x-www-form-urlencoded' | 'multipart/form-data' | 'application/xml' | 'text/xml' | 'text/html' | 'application/octet-stream' | 'application/pdf' | 'application/javascript' | 'application/zip' | 'application/graphql' | 'application/ld+json' | 'application/vnd.api+json';
  let bodyType = $state<BodyType>('application/json');

  // Initialize from headers ONCE
  let bodyTypeInitialized = false;
  $effect(() => {
    if (!bodyTypeInitialized) {
      const ct = (endpointStore.requestConfig.headers?.['Content-Type'] as string | undefined) ?? undefined;
      if (ct) bodyType = ct as any;
      bodyTypeInitialized = true;
    }
  });

  async function handleSendRequest() { await endpointStore.sendRequest(); }

  function clearBody() { endpointStore.updateRequestBody(''); }

  const fullUrl = $derived(() => {
    if (!selectedEndpoint || !currentEnvironment) return '';
    return `${currentEnvironment.baseUrl}${selectedEndpoint.path}`;
  });

  // ----------------------
  // Parameters editor state (Key/Value only)
  // ----------------------
  type ParamRow = { id: string; key: string; value: string; enabled: boolean };

  function uid() { try { return crypto.randomUUID(); } catch { return `id_${Date.now().toString(36)}_${Math.random().toString(36).slice(2,7)}`; } }

  let paramRows = $state<ParamRow[]>([]);
  let paramsInitialized = false;

  // Initialize from store params ONCE
  $effect(() => {
    if (!paramsInitialized) {
      const entries = Object.entries(requestConfig.params ?? {});
      paramRows = entries.map(([k, v]) => ({ id: uid(), key: k, value: String(v ?? ''), enabled: true }));
      paramsInitialized = true;
    }
  });

  function shallowEqual(a: Record<string, string> | undefined, b: Record<string, string>): boolean {
    if (!a) return Object.keys(b).length === 0;
    const aKeys = Object.keys(a);
    const bKeys = Object.keys(b);
    if (aKeys.length !== bKeys.length) return false;
    for (const k of aKeys) { if (a[k] !== b[k]) return false; }
    return true;
  }

  function syncRowsToStore() {
    const obj: Record<string, string> = {};
    for (const r of paramRows) { if (r.enabled && r.key.trim()) obj[r.key] = r.value; }
    if (!shallowEqual(endpointStore.requestConfig.params as any, obj)) {
      endpointStore.requestConfig.params = obj;
    }
  }

  function updateRow(id: string, field: keyof ParamRow, value: string | boolean) {
    paramRows = paramRows.map(r => r.id === id ? { ...r, [field]: value } as ParamRow : r);
    syncRowsToStore();
  }

  function addRow(afterId?: string) {
    const newRow: ParamRow = { id: uid(), key: '', value: '', enabled: true };
    if (!afterId) { paramRows = [...paramRows, newRow]; }
    else {
      const idx = paramRows.findIndex(r => r.id === afterId);
      if (idx >= 0) paramRows = [...paramRows.slice(0, idx + 1), newRow, ...paramRows.slice(idx + 1)];
      else paramRows = [...paramRows, newRow];
    }
  }

  function removeRow(id: string) { paramRows = paramRows.filter(r => r.id !== id); syncRowsToStore(); }

  // DnD for params
  let dragIndex: number | null = null;
  function onDragStart(index: number) { dragIndex = index; }
  function onDragOver(e: DragEvent, overIndex: number) { e.preventDefault(); if (dragIndex === null || dragIndex === overIndex) return; }
  function onDrop(e: DragEvent, dropIndex: number) {
    e.preventDefault(); if (dragIndex === null || dragIndex === dropIndex) return;
    const rows = [...paramRows]; const [moved] = rows.splice(dragIndex, 1); rows.splice(dropIndex, 0, moved);
    paramRows = rows; dragIndex = null; syncRowsToStore();
  }

  // ----------------------
  // Headers editor state (same template as params)
  // ----------------------
  type HeaderRow = { id: string; key: string; value: string; enabled: boolean };
  let headerRows = $state<HeaderRow[]>([]);
  let headersInitialized = false;

  // Initialize from store headers ONCE
  $effect(() => {
    if (!headersInitialized) {
      const entries = Object.entries(requestConfig.headers ?? {});
      headerRows = entries.map(([k, v]) => ({ id: uid(), key: k, value: String(v ?? ''), enabled: true }));
      headersInitialized = true;
    }
  });

  function syncHeaderRowsToStore() {
    const obj: Record<string, string> = {};
    for (const r of headerRows) { if (r.enabled && r.key.trim()) obj[r.key] = r.value; }
    if (!shallowEqual(endpointStore.requestConfig.headers as any, obj)) {
      endpointStore.requestConfig.headers = obj;
    }
  }

  function updateHeaderRow(id: string, field: keyof HeaderRow, value: string | boolean) {
    headerRows = headerRows.map(r => r.id === id ? { ...r, [field]: value } as HeaderRow : r);
    syncHeaderRowsToStore();
  }

  function addHeaderRow(afterId?: string) {
    const newRow: HeaderRow = { id: uid(), key: '', value: '', enabled: true };
    if (!afterId) { headerRows = [...headerRows, newRow]; }
    else {
      const idx = headerRows.findIndex(r => r.id === afterId);
      if (idx >= 0) headerRows = [...headerRows.slice(0, idx + 1), newRow, ...headerRows.slice(idx + 1)];
      else headerRows = [...headerRows, newRow];
    }
  }

  function removeHeaderRow(id: string) { headerRows = headerRows.filter(r => r.id !== id); syncHeaderRowsToStore(); }

  // DnD for headers
  let headerDragIndex: number | null = null;
  function onHeaderDragStart(index: number) { headerDragIndex = index; }
  function onHeaderDragOver(e: DragEvent, overIndex: number) { e.preventDefault(); if (headerDragIndex === null || headerDragIndex === overIndex) return; }
  function onHeaderDrop(e: DragEvent, dropIndex: number) {
    e.preventDefault(); if (headerDragIndex === null || headerDragIndex === dropIndex) return;
    const rows = [...headerRows]; const [moved] = rows.splice(headerDragIndex, 1); rows.splice(dropIndex, 0, moved);
    headerRows = rows; headerDragIndex = null; syncHeaderRowsToStore();
  }

  function onBodyChange(val: string) {
    let next = val;
    if (bodyType === 'application/json') {
      try {
        next = JSON.stringify(JSON.parse(val || 'null'), null, 2);
      } catch {}
    }
    if (next !== requestConfig.body) {
      endpointStore.updateRequestBody(next);
    }
  }

  function setBodyType(t: BodyType) {
    if (bodyType === t) return;
    bodyType = t;
    // If a Content-Type header row exists, update it to match
    const idx = headerRows.findIndex(r => r.key.toLowerCase() === 'content-type');
    if (idx >= 0 && headerRows[idx].value !== t) {
      headerRows = headerRows.map((r, i) => i === idx ? { ...r, value: t } : r);
      syncHeaderRowsToStore();
    }
  }
</script>

<div class="flex h-full flex-col bg-[#0b0b0b]">
  {#if selectedEndpoint}
    <!-- Request Tabs Strip -->
    <div class="bg-[#0a0a0a] px-3">
      <div class="flex items-center gap-2">
        <button class="border-b-2 border-indigo-500 px-3 py-2 text-sm font-medium text-gray-100">{selectedEndpoint.method.toUpperCase()} <span class="ml-2 truncate max-w-[240px] align-middle">{selectedEndpoint.path || 'Untitled'}</span></button>
        <Button variant="ghost" size="icon" class="text-gray-400 hover:text-gray-200"><Plus /></Button>
      </div>
    </div>

    <!-- Request Bar (flat) -->
    <div class="bg-[#0a0a0a] p-3">
      <div class="rounded-lg bg-[#0f0f0f] p-2">
        <div class="flex items-center gap-2">
          <MethodSelector method={selectedEndpoint.method} readonly={true} />
          <Input class="flex-1 border-0 bg-[#111111] text-gray-200" value={fullUrl()} readonly />
          <Button class="bg-indigo-600 hover:bg-indigo-700" disabled={isLoading} onclick={handleSendRequest}>
            {#if isLoading}
              <span class="size-4 animate-spin rounded-full border-2 border-white/30 border-t-white"></span>
              <span>Sending…</span>
            {:else}
              <span>Send</span>
            {/if}
          </Button>
          <Button variant="secondary" class="text-gray-200">Save</Button>
        </div>
        {#if error}
          <div class="mt-2 rounded-md border border-red-900/40 bg-red-900/10 px-3 py-2 text-sm text-red-400">⚠️ {error}</div>
        {/if}
      </div>
    </div>

    <!-- Tabs + Content -->
    <div class="flex flex-1 flex-col overflow-hidden">
      <Tabs value={activeTab} onValueChange={(v) => (activeTab = v as typeof activeTab)} class="flex-1">
        <TabsList class="rounded-none bg-transparent p-0">
          <div class="flex">
            <TabsTrigger value="parameters" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Parameters</TabsTrigger>
            <TabsTrigger value="body" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Body</TabsTrigger>
            <TabsTrigger value="headers" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Headers</TabsTrigger>
            <TabsTrigger value="authorization" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Authorization</TabsTrigger>
            <TabsTrigger value="prerequest" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Pre-request Script</TabsTrigger>
            <TabsTrigger value="postrequest" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Post-request Script</TabsTrigger>
            <TabsTrigger value="variables" class="rounded-none border-b-2 border-transparent px-4 py-3 text-sm text-gray-300 hover:text-gray-100 data-[state=active]:border-indigo-500 data-[state=active]:text-indigo-400">Variables</TabsTrigger>
          </div>
        </TabsList>

        <!-- Parameters Tab -->
        <TabsContent value="parameters" class="flex-1 overflow-y-auto p-4">
          <div class="space-y-3">
            <div class="flex items-center justify-between text-sm text-gray-400">
              <div>Query Parameters</div>
              <Button variant="ghost" size="icon" class="text-gray-400 hover:text-gray-100" title="Add parameter" onclick={() => addRow()}><Plus /></Button>
            </div>

            <!-- Header Row -->
            <div class="grid grid-cols-[2rem_2rem_1fr_1fr_3.5rem] items-center gap-2 px-3 py-2 text-xs text-gray-400">
              <div></div>
              <div></div>
              <div>Key</div>
              <div>Value</div>
              <div></div>
            </div>

            <!-- Rows -->
            <div class="divide-y divide-gray-900/50">
              {#each paramRows as row, index (row.id)}
                <div
                  class="grid grid-cols-[2rem_2rem_1fr_1fr_3.5rem] items-center gap-2 px-3 py-2"
                  role="button"
                  tabindex="0"
                  ondragover={(e) => onDragOver(e, index)}
                  ondrop={(e) => onDrop(e, index)}
                >
                  <button class="text-gray-500 hover:text-gray-300 cursor-grab active:cursor-grabbing" title="Drag to reorder" aria-label="Drag handle" draggable={Boolean(row.key || row.value)} ondragstart={() => onDragStart(index)} ondragend={() => (dragIndex = null)}>
                    <GripVertical class="size-4" />
                  </button>
                  <input type="checkbox" class="h-4 w-4 rounded border-gray-700 bg-transparent" checked={row.enabled} onchange={(e) => updateRow(row.id, 'enabled', (e.currentTarget as HTMLInputElement).checked)} />
                  <Input class="h-9 border-0 bg-[#151515] text-gray-200" placeholder="Key" value={row.key} oninput={(e) => updateRow(row.id, 'key', (e.currentTarget as HTMLInputElement).value)} />
                  <Input class="h-9 border-0 bg-[#151515] text-gray-200" placeholder="Value" value={row.value} oninput={(e) => updateRow(row.id, 'value', (e.currentTarget as HTMLInputElement).value)} />
                  <div class="flex items-center justify-end gap-1">
                    <Button variant="ghost" size="icon" class="text-gray-400 hover:text-gray-200" title="Add row below" onclick={() => addRow(row.id)}><Plus /></Button>
                    <Button variant="ghost" size="icon" class="text-red-500 hover:text-red-400" title="Delete" onclick={() => removeRow(row.id)}><Trash2 /></Button>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </TabsContent>

        <!-- Body Tab -->
        <TabsContent value="body" class="flex-1 overflow-hidden p-4">
          <div class="flex h-full flex-col gap-3">
            <div class="flex items-center gap-2">
              <span class="text-xs text-gray-400">Content-Type</span>
              <Select type="single" value={bodyType} onValueChange={(v) => setBodyType(v as BodyType)}>
                <SelectTrigger class="h-8 rounded-md border border-gray-800 bg-[#121212] text-gray-200 px-2">
                  <span class="text-sm">{bodyType}</span>
                </SelectTrigger>
                <SelectContent class="min-w-[300px] border border-gray-800 bg-[#111] text-gray-200">
                  <SelectItem value="application/json">application/json</SelectItem>
                  <SelectItem value="text/plain">text/plain</SelectItem>
                  <SelectItem value="application/x-www-form-urlencoded">application/x-www-form-urlencoded</SelectItem>
                  <SelectItem value="multipart/form-data">multipart/form-data</SelectItem>
                  <SelectItem value="application/xml">application/xml</SelectItem>
                  <SelectItem value="text/xml">text/xml</SelectItem>
                  <SelectItem value="text/html">text/html</SelectItem>
                  <SelectItem value="application/octet-stream">application/octet-stream</SelectItem>
                  <SelectItem value="application/pdf">application/pdf</SelectItem>
                  <SelectItem value="application/javascript">application/javascript</SelectItem>
                  <SelectItem value="application/zip">application/zip</SelectItem>
                  <SelectItem value="application/graphql">application/graphql</SelectItem>
                  <SelectItem value="application/ld+json">application/ld+json</SelectItem>
                  <SelectItem value="application/vnd.api+json">application/vnd.api+json</SelectItem>
                </SelectContent>
              </Select>
              <Button variant="ghost" size="icon" class="ml-1 text-gray-400 hover:text-gray-100" title="Clear body" onclick={clearBody}>
                <Trash2 class="size-4" />
              </Button>
            </div>
            <div class="flex-1 overflow-hidden rounded-lg border border-gray-900/50">
              <CodeMirror className="h-full" value={requestConfig.body} language={bodyType === 'application/json' ? 'json' : 'text'} placeholder={bodyType === 'application/json' ? '{\n  "key": "value"\n}' : 'Body'} onChange={onBodyChange} />
            </div>
          </div>
        </TabsContent>

        <!-- Headers Tab -->
        <TabsContent value="headers" class="flex-1 overflow-y-auto p-4">
          <div class="space-y-3">
            <div class="flex items-center justify-between text-sm text-gray-400">
              <div>Headers</div>
              <Button variant="ghost" size="icon" class="text-gray-400 hover:text-gray-100" title="Add header" onclick={() => addHeaderRow()}><Plus /></Button>
            </div>

            <div class="grid grid-cols-[2rem_2rem_1fr_1fr_3.5rem] items-center gap-2 px-3 py-2 text-xs text-gray-400">
              <div></div>
              <div></div>
              <div>Key</div>
              <div>Value</div>
              <div></div>
            </div>

            <div class="divide-y divide-gray-900/50">
              {#each headerRows as row, index (row.id)}
                <div
                  class="grid grid-cols-[2rem_2rem_1fr_1fr_3.5rem] items-center gap-2 px-3 py-2"
                  role="button"
                  tabindex="0"
                  ondragover={(e) => onHeaderDragOver(e, index)}
                  ondrop={(e) => onHeaderDrop(e, index)}
                >
                  <button class="text-gray-500 hover:text-gray-300 cursor-grab active:cursor-grabbing" title="Drag to reorder" aria-label="Drag handle" draggable={Boolean(row.key || row.value)} ondragstart={() => onHeaderDragStart(index)} ondragend={() => (headerDragIndex = null)}>
                    <GripVertical class="size-4" />
                  </button>
                  <input type="checkbox" class="h-4 w-4 rounded border-gray-700 bg-transparent" checked={row.enabled} onchange={(e) => updateHeaderRow(row.id, 'enabled', (e.currentTarget as HTMLInputElement).checked)} />
                  <Input class="h-9 border-0 bg-[#151515] text-gray-200" placeholder="Key" value={row.key} oninput={(e) => updateHeaderRow(row.id, 'key', (e.currentTarget as HTMLInputElement).value)} />
                  <Input class="h-9 border-0 bg-[#151515] text-gray-200" placeholder="Value" value={row.value} oninput={(e) => updateHeaderRow(row.id, 'value', (e.currentTarget as HTMLInputElement).value)} />
                  <div class="flex items-center justify-end gap-1">
                    <Button variant="ghost" size="icon" class="text-gray-400 hover:text-gray-200" title="Add row below" onclick={() => addHeaderRow(row.id)}><Plus /></Button>
                    <Button variant="ghost" size="icon" class="text-red-500 hover:text-red-400" title="Delete" onclick={() => removeHeaderRow(row.id)}><Trash2 /></Button>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </TabsContent>

        <!-- Placeholder Tabs -->
        <TabsContent value="authorization" class="p-4 text-sm text-gray-400">Authorization configuration coming soon.</TabsContent>
        <TabsContent value="prerequest" class="p-4 text-sm text-gray-400">Pre-request script support coming soon.</TabsContent>
        <TabsContent value="postrequest" class="p-4 text-sm text-gray-400">Post-request script support coming soon.</TabsContent>
        <TabsContent value="variables" class="p-4 text-sm text-gray-400">Workspace variables coming soon.</TabsContent>
      </Tabs>

      <!-- Bottom info panel -->
      <div class="flex items-center justify-center py-10">
        <div class="flex flex-col items-center gap-3 text-sm text-gray-400">
          <div class="grid grid-cols-2 gap-x-12 gap-y-2">
            <div class="flex items-center gap-3"><span>Send Request</span><span><kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">⌘</kbd> <kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">↩</kbd></span></div>
            <div class="flex items-center gap-3"><span>Keyboard shortcuts</span><span><kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">⌘</kbd> <kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">/</kbd></span></div>
            <div class="flex items-center gap-3"><span>Search & command menu</span><span><kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">⌘</kbd> <kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">K</kbd></span></div>
            <div class="flex items-center gap-3"><span>Help menu</span><span><kbd class="rounded border border-gray-700 bg-[#111] px-1.5 py-0.5">?</kbd></span></div>
          </div>
          <Button variant="outline" class="mt-2 border-gray-800 text-gray-200"><span>Documentation</span><ExternalLink class="ml-2 size-4" /></Button>
        </div>
      </div>
    </div>

  {:else}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-8 text-center">
      <div class="text-5xl opacity-50">📡</div>
      <div class="text-lg font-medium text-gray-100">Select an endpoint to get started</div>
      <div class="max-w-md text-sm leading-relaxed text-gray-400">Choose an endpoint from the sidebar to build and test requests</div>
    </div>
  {/if}
</div>
