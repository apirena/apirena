<script lang="ts">
  let visible = $state(false);
  let message = $state<string | null>(null);
  let stack = $state<string | null>(null);

  function showError(msg: string, stk?: string) {
    visible = true;
    message = msg;
    stack = stk ?? null;
    console.error('[GlobalErrorOverlay]', msg, stk);
  }

  if (typeof window !== 'undefined') {
    window.addEventListener('error', (e: ErrorEvent) => {
      try { e.preventDefault?.(); } catch {}
      const stk = e.error && (e.error as any).stack ? (e.error as any).stack as string : undefined;
      const msg = e.message ?? 'Unknown error';
      showError(msg, stk);
    });

    window.addEventListener('unhandledrejection', (e: PromiseRejectionEvent) => {
      try { e.preventDefault?.(); } catch {}
      const reason: any = e.reason;
      const msg = typeof reason === 'string' ? reason : (reason?.message ?? 'Unhandled rejection');
      const stk = reason?.stack ?? undefined;
      showError(msg, stk);
    });
  }

  function close() {
    visible = false;
  }
</script>

{#if visible}
  <div class="overlay" role="alertdialog" aria-live="assertive">
    <div class="panel">
      <div class="header">
        <div class="title">💥 Runtime Error</div>
        <button class="close" onclick={close} aria-label="Close">✕</button>
      </div>
      <div class="body">
        <div class="message">{message}</div>
        {#if stack}
          <pre class="stack">{stack}</pre>
        {/if}
        <div class="hint">Check the DevTools console for details. This overlay prevents the app from closing so you can inspect the error.</div>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
  }
  .panel {
    width: min(900px, 95vw);
    max-height: 85vh;
    background: var(--color-surface, #111214);
    color: var(--color-text-primary, #fff);
    border: 1px solid var(--color-border, #333);
    border-radius: 10px;
    box-shadow: 0 10px 40px rgba(0,0,0,0.5);
    display: flex;
    flex-direction: column;
  }
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 10px 14px;
    border-bottom: 1px solid var(--color-border, #333);
    background: var(--color-surface-secondary, #191a1d);
  }
  .title {
    font-weight: 600;
  }
  .close {
    border: none;
    background: transparent;
    color: inherit;
    font-size: 16px;
    cursor: pointer;
    opacity: 0.8;
  }
  .close:hover { opacity: 1; }
  .body {
    padding: 12px 14px 16px;
    overflow: auto;
  }
  .message {
    font-weight: 600;
    margin-bottom: 8px;
    color: #ffb4b4;
  }
  .stack {
    margin: 0;
    white-space: pre-wrap;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 12px;
    line-height: 1.4;
    color: #ddd;
    background: rgba(255,255,255,0.04);
    border: 1px solid var(--color-border, #333);
    border-radius: 8px;
    padding: 10px;
  }
  .hint { margin-top: 10px; font-size: 12px; opacity: 0.8; }
</style>
