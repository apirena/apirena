<script lang="ts">
  import { endpointStore } from '../../stores/endpoints.svelte.js';
  import MethodSelector from './MethodSelector.svelte';
  import ParamEditor from './ParamEditor.svelte';
  import HeaderEditor from './HeaderEditor.svelte';

  let selectedEndpoint = $derived(endpointStore.selectedEndpoint);
  let requestConfig = $derived(endpointStore.requestConfig);
  let isLoading = $derived(endpointStore.isLoading);
  let error = $derived(endpointStore.error);

  async function handleSendRequest() {
    await endpointStore.sendRequest();
  }

  // Extract URL segments that might be dynamic parameters
  const dynamicSegments = $derived(() => {
    if (!selectedEndpoint) return [];
    
    return selectedEndpoint.path
      .split('/')
      .filter(segment => segment.startsWith(':') || (segment.startsWith('{') && segment.endsWith('}')))
      .map(segment => segment.replace(/^[:{}]|[{}]$/g, ''));
  });
</script>

<div class="request-builder">
  {#if selectedEndpoint}
    <div class="request-header">
      <h2>Request Builder</h2>
      <div class="endpoint-info">
        <MethodSelector 
          method={selectedEndpoint.method}
          readonly={true}
        />
        <div class="endpoint-path">
          {selectedEndpoint.path}
        </div>
      </div>
    </div>

    <div class="request-body">
      <div class="tabs">
        <div class="tab-list">
          <button class="tab active">Params</button>
          <button class="tab">Headers</button>
          <button class="tab">Body</button>
        </div>
        
        <div class="tab-content">
          <div class="tab-panel">
            <ParamEditor
              params={requestConfig.params}
              onUpdateParam={(key, value) => endpointStore.updateRequestParam(key, value)}
              suggestedParams={dynamicSegments()}
            />
          </div>
        </div>
      </div>
    </div>

    <div class="request-actions">
      {#if error}
        <div class="error-message">
          ⚠️ {error}
        </div>
      {/if}
      
      <button 
        class="send-button"
        onclick={handleSendRequest}
        disabled={isLoading}
      >
        {#if isLoading}
          <div class="spinner"></div>
          Sending...
        {:else}
          🚀 Send Request
        {/if}
      </button>
    </div>

  {:else}
    <div class="no-selection">
      <div class="no-selection-icon">📡</div>
      <div class="no-selection-text">Select an endpoint to get started</div>
      <div class="no-selection-hint">
        Choose an endpoint from the sidebar to build and test requests
      </div>
    </div>
  {/if}
</div>

<style>
  .request-builder {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--color-surface);
  }

  .request-header {
    padding: 1rem;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface-secondary);
  }

  .request-header h2 {
    margin: 0 0 0.75rem 0;
    font-size: 1.2rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .endpoint-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .endpoint-path {
    flex: 1;
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', monospace;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    padding: 0.5rem 0.75rem;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 6px;
  }

  .request-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .tabs {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .tab-list {
    display: flex;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface-secondary);
  }

  .tab {
    padding: 0.75rem 1rem;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    border-bottom: 2px solid transparent;
  }

  .tab:hover {
    background: var(--color-surface-hover);
    color: var(--color-text-primary);
  }

  .tab.active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
    background: var(--color-primary-surface);
  }

  .tab-content {
    flex: 1;
    overflow: hidden;
  }

  .tab-panel {
    height: 100%;
    overflow-y: auto;
    padding: 1rem;
  }

  .request-actions {
    padding: 1rem;
    border-top: 1px solid var(--color-border);
    background: var(--color-surface-secondary);
  }

  .error-message {
    margin-bottom: 0.75rem;
    padding: 0.5rem 0.75rem;
    background: var(--color-error-surface);
    color: var(--color-error);
    border: 1px solid var(--color-error-border);
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .send-button {
    width: 100%;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 6px;
    background: var(--color-primary);
    color: white;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .send-button:hover {
    background: var(--color-primary-hover);
  }

  .send-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .no-selection {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 2rem;
    text-align: center;
  }

  .no-selection-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }

  .no-selection-text {
    font-size: 1.25rem;
    color: var(--color-text-primary);
    margin-bottom: 0.5rem;
    font-weight: 500;
  }

  .no-selection-hint {
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    max-width: 300px;
    line-height: 1.4;
  }
</style>
