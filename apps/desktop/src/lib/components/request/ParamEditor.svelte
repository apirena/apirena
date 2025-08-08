<script lang="ts">
  interface Props {
    params: Record<string, string>;
    onUpdateParam: (key: string, value: string) => void;
    suggestedParams?: string[];
  }

  let { params, onUpdateParam, suggestedParams = [] }: Props = $props();

  // Convert params object to array for easier manipulation
  const paramEntries = $derived(() => {
    const entries = Object.entries(params);
    // Always show at least one empty row for new params
    if (entries.length === 0 || entries[entries.length - 1][1] !== '') {
      entries.push(['', '']);
    }
    return entries;
  });

  function updateParam(index: number, key: string, value: string) {
    const entries = paramEntries();
    const oldKey = entries[index]?.[0];
    
    // Remove old key if it changed
    if (oldKey && oldKey !== key) {
      delete params[oldKey];
    }
    
    // Set new value if key is provided
    if (key.trim()) {
      onUpdateParam(key, value);
    }
  }

  function removeParam(key: string) {
    delete params[key];
    // Trigger reactivity by creating new object
    params = { ...params };
  }

  function addSuggestedParam(paramName: string) {
    onUpdateParam(paramName, '');
  }
</script>

<div class="param-editor">
  <div class="section-header">
    <h3>Parameters</h3>
    {#if suggestedParams.length > 0}
      <div class="suggested-params">
        <span class="suggested-label">Suggested:</span>
        {#each suggestedParams as paramName}
          {#if !params[paramName]}
            <button 
              class="suggested-param-btn"
              onclick={() => addSuggestedParam(paramName)}
            >
              + {paramName}
            </button>
          {/if}
        {/each}
      </div>
    {/if}
  </div>

  <div class="param-list">
    <div class="param-header">
      <div class="param-key-header">Key</div>
      <div class="param-value-header">Value</div>
      <div class="param-actions-header"></div>
    </div>

    {#each paramEntries() as [key, value], index}
      <div class="param-row" class:empty={!key && !value}>
        <input
          class="param-key"
          type="text"
          placeholder="Parameter name"
          value={key}
          oninput={(e) => updateParam(index, e.currentTarget.value, value)}
        />
        <input
          class="param-value"
          type="text"
          placeholder="Parameter value"
          value={value}
          oninput={(e) => updateParam(index, key, e.currentTarget.value)}
        />
        <div class="param-actions">
          {#if key}
            <button 
              class="remove-param-btn"
              onclick={() => removeParam(key)}
              title="Remove parameter"
            >
              ✕
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>

  {#if Object.keys(params).length === 0}
    <div class="empty-state">
      <div class="empty-icon">🔧</div>
      <div class="empty-text">No parameters yet</div>
      <div class="empty-hint">Add query parameters or path variables for your request</div>
    </div>
  {/if}
</div>

<style>
  .param-editor {
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

  .suggested-params {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .suggested-label {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
  }

  .suggested-param-btn {
    padding: 0.25rem 0.5rem;
    border: 1px dashed var(--color-primary);
    border-radius: 4px;
    background: transparent;
    color: var(--color-primary);
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .suggested-param-btn:hover {
    background: var(--color-primary-surface);
    border-style: solid;
  }

  .param-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .param-header {
    display: grid;
    grid-template-columns: 1fr 1fr auto;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-surface-secondary);
    border-radius: 6px 6px 0 0;
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--color-text-secondary);
  }

  .param-row {
    display: grid;
    grid-template-columns: 1fr 1fr auto;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    transition: all 0.2s;
  }

  .param-row:hover {
    background: var(--color-surface-hover);
  }

  .param-row.empty {
    opacity: 0.7;
  }

  .param-key,
  .param-value {
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    background: var(--color-surface);
    color: var(--color-text-primary);
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .param-key:focus,
  .param-value:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(var(--color-primary-rgb), 0.1);
  }

  .param-key {
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', monospace;
  }

  .param-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
  }

  .remove-param-btn {
    padding: 0.25rem;
    border: none;
    background: transparent;
    color: var(--color-error);
    cursor: pointer;
    border-radius: 3px;
    transition: all 0.2s;
    opacity: 0.7;
  }

  .remove-param-btn:hover {
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
