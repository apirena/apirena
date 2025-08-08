<script lang="ts">
  interface Props {
    headers: Record<string, string>;
    onUpdateHeader: (key: string, value: string) => void;
  }

  let { headers, onUpdateHeader }: Props = $props();

  // Convert headers object to array for easier manipulation
  const headerEntries = $derived(() => {
    const entries = Object.entries(headers);
    // Always show at least one empty row for new headers
    if (entries.length === 0 || entries[entries.length - 1][1] !== '') {
      entries.push(['', '']);
    }
    return entries;
  });

  function updateHeader(index: number, key: string, value: string) {
    const entries = headerEntries();
    const oldKey = entries[index]?.[0];
    
    // Remove old key if it changed
    if (oldKey && oldKey !== key) {
      delete headers[oldKey];
    }
    
    // Set new value if key is provided
    if (key.trim()) {
      onUpdateHeader(key, value);
    }
  }

  function removeHeader(key: string) {
    delete headers[key];
    // Trigger reactivity by creating new object
    headers = { ...headers };
  }

  const commonHeaders = [
    'Content-Type',
    'Authorization',
    'Accept',
    'User-Agent',
    'X-API-Key',
    'Cache-Control'
  ];

  function addCommonHeader(headerName: string) {
    let defaultValue = '';
    
    switch (headerName) {
      case 'Content-Type':
        defaultValue = 'application/json';
        break;
      case 'Accept':
        defaultValue = 'application/json';
        break;
      case 'Authorization':
        defaultValue = 'Bearer ';
        break;
      default:
        defaultValue = '';
    }
    
    onUpdateHeader(headerName, defaultValue);
  }
</script>

<div class="header-editor">
  <div class="section-header">
    <h3>Headers</h3>
    <div class="common-headers">
      <span class="common-label">Common:</span>
      {#each commonHeaders as headerName}
        {#if !headers[headerName]}
          <button 
            class="common-header-btn"
            onclick={() => addCommonHeader(headerName)}
          >
            + {headerName}
          </button>
        {/if}
      {/each}
    </div>
  </div>

  <div class="header-list">
    <div class="header-header">
      <div class="header-key-header">Header</div>
      <div class="header-value-header">Value</div>
      <div class="header-actions-header"></div>
    </div>

    {#each headerEntries() as [key, value], index}
      <div class="header-row" class:empty={!key && !value}>
        <input
          class="header-key"
          type="text"
          placeholder="Header name"
          value={key}
          oninput={(e) => updateHeader(index, e.currentTarget.value, value)}
        />
        <input
          class="header-value"
          type="text"
          placeholder="Header value"
          value={value}
          oninput={(e) => updateHeader(index, key, e.currentTarget.value)}
        />
        <div class="header-actions">
          {#if key}
            <button 
              class="remove-header-btn"
              onclick={() => removeHeader(key)}
              title="Remove header"
            >
              ✕
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>

  {#if Object.keys(headers).length === 0}
    <div class="empty-state">
      <div class="empty-icon">📋</div>
      <div class="empty-text">No headers set</div>
      <div class="empty-hint">Add HTTP headers for authentication, content type, etc.</div>
    </div>
  {/if}
</div>

<style>
  .header-editor {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
    gap: 1rem;
  }

  .section-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .common-headers {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .common-label {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
  }

  .common-header-btn {
    padding: 0.25rem 0.5rem;
    border: 1px dashed var(--color-primary);
    border-radius: 4px;
    background: transparent;
    color: var(--color-primary);
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .common-header-btn:hover {
    background: var(--color-primary-surface);
    border-style: solid;
  }

  .header-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .header-header {
    display: grid;
    grid-template-columns: 1fr 2fr auto;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-surface-secondary);
    border-radius: 6px 6px 0 0;
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--color-text-secondary);
  }

  .header-row {
    display: grid;
    grid-template-columns: 1fr 2fr auto;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    transition: all 0.2s;
  }

  .header-row:hover {
    background: var(--color-surface-hover);
  }

  .header-row.empty {
    opacity: 0.7;
  }

  .header-key,
  .header-value {
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    background: var(--color-surface);
    color: var(--color-text-primary);
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .header-key:focus,
  .header-value:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(var(--color-primary-rgb), 0.1);
  }

  .header-key {
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', monospace;
  }

  .header-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
  }

  .remove-header-btn {
    padding: 0.25rem;
    border: none;
    background: transparent;
    color: var(--color-error);
    cursor: pointer;
    border-radius: 3px;
    transition: all 0.2s;
    opacity: 0.7;
  }

  .remove-header-btn:hover {
    background: var(--color-error-surface);
    opacity: 1;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem 1rem;
    text-align: center;
  }

  .empty-icon {
    font-size: 2rem;
    margin-bottom: 0.5rem;
    opacity: 0.5;
  }

  .empty-text {
    font-size: 1rem;
    color: var(--color-text-primary);
    margin-bottom: 0.25rem;
  }

  .empty-hint {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
    line-height: 1.4;
  }
</style>
