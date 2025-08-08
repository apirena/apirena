<script lang="ts">
  import { endpointStore } from '../../stores/endpoints.svelte.js';
  import { Card, CardContent, CardHeader } from "$lib/components/ui/card/index.js";
  import { Tabs, TabsContent, TabsList, TabsTrigger } from "$lib/components/ui/tabs/index.js";

  let lastResponse = $derived(endpointStore.lastResponse);
  let isLoading = $derived(endpointStore.isLoading);

  // Format response body for display
  const formattedBody = $derived(() => {
    if (!lastResponse?.body) return '';
    
    try {
      const parsed = JSON.parse(lastResponse.body);
      return JSON.stringify(parsed, null, 2);
    } catch {
      return lastResponse.body;
    }
  });

  // Determine response type
  const responseType = $derived(() => {
    if (!lastResponse) return 'none';
    
    const contentType = lastResponse.headers['content-type'] || '';
    if (contentType.includes('application/json')) return 'json';
    if (contentType.includes('text/html')) return 'html';
    if (contentType.includes('text/')) return 'text';
    return 'raw';
  });

  // Get status color
  const statusColor = $derived(() => {
    if (!lastResponse) return '#6B7280';
    
    const status = lastResponse.status;
    if (status >= 200 && status < 300) return '#10B981'; // Green
    if (status >= 300 && status < 400) return '#F59E0B'; // Amber
    if (status >= 400 && status < 500) return '#EF4444'; // Red
    if (status >= 500) return '#DC2626'; // Dark Red
    return '#6B7280'; // Gray
  });

  // Get status text
  const statusText = $derived(() => {
    if (!lastResponse) return '';
    
    const status = lastResponse.status;
    if (status >= 200 && status < 300) return 'Success';
    if (status >= 300 && status < 400) return 'Redirect';
    if (status >= 400 && status < 500) return 'Client Error';
    if (status >= 500) return 'Server Error';
    return 'Unknown';
  });

  let activeTab = $state<'body' | 'headers'>('body');
</script>

<Card class="h-full rounded-none border-0">
  {#if isLoading}
    <CardContent class="flex h-full flex-col items-center justify-center gap-4">
      <div class="size-8 animate-spin rounded-full border-2 border-border border-t-primary"></div>
      <div class="text-sm text-muted-foreground">Sending request...</div>
    </CardContent>
  {:else if lastResponse}
    <CardHeader class="border-b bg-muted/40 p-4">
      <div class="flex items-center justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="min-w-12 rounded-md border-2 px-2 py-1 text-center font-bold text-lg" style="color:{statusColor};border-color:{statusColor}">
            {lastResponse.status}
          </div>
          <div class="flex flex-col">
            <div class="font-medium" style="color:{statusColor}">{statusText}</div>
            <div class="text-xs text-muted-foreground">{lastResponse.duration_ms}ms</div>
          </div>
        </div>
      </div>
    </CardHeader>

    <CardContent class="p-0 h-[calc(100%-64px)]">
      <Tabs value={activeTab} onValueChange={(v) => (activeTab = v as typeof activeTab)} class="h-full flex flex-col">
        <TabsList class="mx-4 mt-3 w-max">
          <TabsTrigger value="body">Body</TabsTrigger>
          <TabsTrigger value="headers">Headers ({Object.keys(lastResponse.headers).length})</TabsTrigger>
        </TabsList>
        <TabsContent value="body" class="flex-1 overflow-auto p-4">
          {#if lastResponse.body}
            <pre class="m-0 whitespace-pre-wrap break-words text-[13px] leading-5 font-mono {responseType}">{formattedBody}</pre>
          {:else}
            <div class="flex h-full items-center justify-center text-center">
              <div>
                <div class="mb-3 text-4xl opacity-50">📄</div>
                <div class="mb-1 text-lg font-medium">Empty response body</div>
                <div class="text-sm text-muted-foreground">No content returned</div>
              </div>
            </div>
          {/if}
        </TabsContent>
        <TabsContent value="headers" class="flex-1 overflow-auto p-2">
          <div class="divide-y rounded-md border">
            {#each Object.entries(lastResponse.headers) as [key, value]}
              <div class="flex gap-4 p-2">
                <div class="min-w-40 shrink-0 font-mono font-semibold">{key}</div>
                <div class="font-mono text-muted-foreground break-all">{value}</div>
              </div>
            {/each}
          </div>
        </TabsContent>
      </Tabs>
    </CardContent>
  {:else}
    <CardContent class="flex h-full flex-col items-center justify-center text-center gap-2">
      <div class="text-4xl opacity-50">📡</div>
      <div class="text-lg font-medium">No response yet</div>
      <div class="max-w-[320px] text-sm text-muted-foreground">Send a request to see the response here</div>
    </CardContent>
  {/if}
</Card>

<style>
  .json { color: var(--color-syntax-json); }
  .html { color: var(--color-syntax-html); }
</style>
