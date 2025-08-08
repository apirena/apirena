<script lang="ts">
  import { endpointStore } from '../../stores/endpoints.svelte.js';
  import EndpointItem from './EndpointItem.svelte';

  let endpointsByMethod = $derived(endpointStore.endpointsByMethod);
  let selectedEndpoint = $derived(endpointStore.selectedEndpoint);

  const methodColors: Record<string, string> = {
    'Get': '#10B981',      // Green
    'Post': '#F59E0B',     // Amber  
    'Put': '#3B82F6',      // Blue
    'Delete': '#EF4444',   // Red
    'Patch': '#8B5CF6',    // Purple
    'Options': '#6B7280',  // Gray
    'Head': '#6B7280'      // Gray
  };

  const methodOrder = ['Get', 'Post', 'Put', 'Patch', 'Delete', 'Options', 'Head'];
</script>

<div class="endpoint-list">
  {#each methodOrder as method}
    {#if endpointsByMethod[method]?.length > 0}
      <div class="method-group">
        <div class="method-header">
          <span 
            class="method-badge" 
            style="background-color: {methodColors[method]};"
          >
            {method.toUpperCase()}
          </span>
          <span class="method-count">
            {endpointsByMethod[method].length}
          </span>
        </div>
        
        <div class="endpoint-items">
          {#each endpointsByMethod[method] as endpoint (endpoint.path + endpoint.method)}
            <EndpointItem 
              {endpoint}
              isSelected={selectedEndpoint?.path === endpoint.path && 
                          selectedEndpoint?.method === endpoint.method}
              onclick={() => endpointStore.selectEndpoint(endpoint)}
            />
          {/each}
        </div>
      </div>
    {/if}
  {/each}

  {#if Object.keys(endpointsByMethod).length === 0}
    <div class="empty-state">
      <div class="empty-icon">🔍</div>
      <div class="empty-text">No endpoints found</div>
      <div class="empty-hint">
        Select a project to discover API endpoints
      </div>
    </div>
  {/if}
</div>

<style>
  .endpoint-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    background: var(--color-surface);
    min-height: 100%;
  }

  .method-group {
    margin-bottom: 1rem;
  }

  .method-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    margin-bottom: 0.25rem;
    background: var(--color-surface-secondary);
    border-radius: 6px;
    border-left: 3px solid transparent;
  }

  .method-badge {
    display: inline-block;
    padding: 0.125rem 0.375rem;
    border-radius: 3px;
    color: white;
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.025em;
  }

  .method-count {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
    background: var(--color-surface);
    padding: 0.125rem 0.375rem;
    border-radius: 10px;
  }

  .endpoint-items {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    text-align: center;
    height: 200px;
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
  }
</style>
