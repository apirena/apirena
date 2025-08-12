<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  const {
    value = '',
    language = 'json',
    placeholder = '',
    readOnly = false,
    onChange = (val: string) => {},
    className = ''
  } = $props<{
    value?: string;
    language?: 'json' | 'text';
    placeholder?: string;
    readOnly?: boolean;
    onChange?: (val: string) => void;
    className?: string;
  }>();

  let host: HTMLDivElement | null = null;
  let view: any = null;
  let cm: any = null;
  let _prevStructuralDeps: string | null = null;

  async function loadCM() {
    if (cm) return cm;
    const viewMod = await import('@codemirror/view');
    const stateMod = await import('@codemirror/state');
    const cmdMod = await import('@codemirror/commands');
    let jsonMod: any = null;
    try { jsonMod = await import('@codemirror/lang-json'); } catch {}
    cm = { ...viewMod, ...stateMod, ...cmdMod, jsonMod };
    return cm;
  }

  function buildExtensions() {
    const exts: any[] = [];
    if (!cm) return exts;
    exts.push(cm.lineNumbers?.(), cm.highlightActiveLine?.(), cm.history?.(), cm.keymap?.of?.([...(cm.defaultKeymap || []), ...(cm.historyKeymap || [])]));
    exts.push(cm.EditorView?.editable?.of?.(!readOnly));
    // Theme (minimal, dark)
    exts.push(cm.EditorView?.theme?.({
      '&': { height: '100%', backgroundColor: '#111111' },
      '.cm-content': { fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace', color: 'rgb(229 231 235)' },
      '.cm-gutters': { backgroundColor: '#0f0f0f', color: '#6b7280', borderRight: '1px solid #111827' },
      '.cm-activeLineGutter': { backgroundColor: 'transparent' },
      '.cm-activeLine': { backgroundColor: '#1f29371a' },
      '.cm-foldPlaceholder': { background: 'transparent', border: 'none', color: '#9ca3af' },
      '.cm-selectionBackground, ::selection': { backgroundColor: '#6366f166' }
    }, { dark: true }));
    if (placeholder && cm.placeholder) exts.push(cm.placeholder(placeholder));
    if (language === 'json' && cm.jsonMod?.json) exts.push(cm.jsonMod.json());
    return exts.filter(Boolean);
  }

  async function createView() {
    await loadCM();
    if (!host || !cm) return;
    view = new cm.EditorView({
      state: cm.EditorState.create({
        doc: value ?? '',
        extensions: [
          ...buildExtensions(),
          cm.EditorView.updateListener.of((vu: any) => {
            if (vu.docChanged) onChange(vu.state.doc.toString());
          })
        ]
      }),
      parent: host
    });
  }

  onMount(() => { createView(); });
  onDestroy(() => { if (view) view.destroy(); view = null; });

  // External value updates
  $effect(() => {
    if (view && value !== view.state.doc.toString()) {
      view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: value ?? '' } });
    }
  });

  // Recreate editor when structural props change (guard against loops)
  $effect(() => {
    const deps = `${language}|${readOnly}|${placeholder}`;
    if (deps === _prevStructuralDeps) return;
    _prevStructuralDeps = deps;
    if (!host) return;
    if (view) { view.destroy(); view = null; }
    createView();
  });
</script>

<div class={className} style="height: 100%;">
  <div bind:this={host} style="height: 100%;"></div>
</div>
