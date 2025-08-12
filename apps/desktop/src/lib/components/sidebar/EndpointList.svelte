<script lang="ts">
  import { endpointStore } from '../../stores/endpoints.svelte.js';
  import EndpointItem from './EndpointItem.svelte';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import SearchIcon from '@lucide/svelte/icons/search';
  import XIcon from '@lucide/svelte/icons/x';
  import type { Endpoint } from '../../types';

  let projectState = $derived(endpointStore.projectState);
  let selectedEndpoint = $derived(endpointStore.selectedEndpoint);
  
  // Search functionality
  let searchQuery = $state('');
  
  // Get all endpoints sorted alphabetically by path
  let sortedEndpoints = $derived((() => {
    const endpoints = projectState.endpoints || [];
    return [...endpoints].sort((a, b) => a.path.localeCompare(b.path));
  })());
  
  let filteredEndpoints = $derived((() => {
    if (!searchQuery.trim()) {
      return sortedEndpoints;
    }
    
    const query = searchQuery.toLowerCase();
    const filtered = sortedEndpoints.filter(endpoint => 
      endpoint.path.toLowerCase().includes(query) ||
      endpoint.method.toLowerCase().includes(query) ||
      endpoint.handler.toLowerCase().includes(query)
    );
    return filtered;
  })());

  let totalFilteredCount = $derived(filteredEndpoints.length);

  // Strong unique key: prefer endpoint.id if present; else compose with file (handler)
  // Add the per-list index as a final suffix to guarantee uniqueness even in edge cases.
  const endpointKey = (e: any, i: number) => e.id ?? `${e.method}:${e.path}:${e.line}:${e.handler ?? ''}:${i}`;

  function clearSearch() {
    searchQuery = '';
  }
</script>

<div class="endpoint-list">
  <!-- Search Bar -->
  <div class="search-container">
    <div class="search-input-wrapper">
      <SearchIcon class="search-icon" size={16} />
      <Input
        bind:value={searchQuery}
        placeholder="Search endpoints..."
      />
      {#if searchQuery}
        <Button
          variant="ghost"
          size="sm"
          class="clear-button"
          onclick={clearSearch}
        >
          <XIcon size={14} />
        </Button>
      {/if}
    </div>
    
    {#if searchQuery}
      <div class="search-results-info">
        <span class="results-badge">
          {totalFilteredCount} result{totalFilteredCount !== 1 ? 's' : ''}
        </span>
      </div>
    {/if}
  </div>

  <!-- Endpoint List -->
  <div class="endpoint-items">
    {#each filteredEndpoints as endpoint, i (endpointKey(endpoint, i))}
      <EndpointItem 
        {endpoint}
        isSelected={selectedEndpoint?.path === endpoint.path && 
                    selectedEndpoint?.method === endpoint.method}
        onclick={() => endpointStore.selectEndpoint(endpoint)}
      />
    {/each}

    {#if filteredEndpoints.length === 0}
      <div class="empty-state">
        {#if searchQuery}
          <div class="empty-icon">🔍</div>
          <div class="empty-text">No endpoints match "{searchQuery}"</div>
          <div class="empty-hint">
            Try a different search term or 
            <Button variant="ghost" size="sm" onclick={clearSearch}>
              clear search
            </Button>
          </div>
        {:else}
          <div class="empty-icon">🔍</div>
          <div class="empty-text">No endpoints found</div>
          <div class="empty-hint">
            Select a project to discover API endpoints
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .endpoint-list { 
    display: flex; 
    flex-direction: column; 
    height: 100vh; /* Use viewport height for full height */
    max-height: 100vh; /* Ensure it doesn't exceed viewport */
    background: var(--color-surface);
    overflow: hidden; /* Prevent parent from scrolling */
  }
  
  .search-container {
    padding: 0.75rem;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface);
    position: sticky;
    top: 0;
    z-index: 10;
  }
  
  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }
  
  .search-input-wrapper :global(.search-icon) {
    position: absolute;
    left: 0.75rem;
    color: var(--color-text-tertiary);
    pointer-events: none;
    z-index: 1;
  }
  
  .search-input-wrapper :global(input) {
    padding-left: 2.5rem !important;
    padding-right: 2rem !important;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 0.875rem;
    width: 100%;
  }
  
  .search-input-wrapper :global(input:focus) {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px var(--color-primary-alpha);
  }
  
  .search-input-wrapper :global(.clear-button) {
    position: absolute;
    right: 0.25rem;
    padding: 0.25rem;
    height: auto;
    min-height: auto;
  }
  
  .search-results-info {
    margin-top: 0.5rem;
    display: flex;
    justify-content: center;
  }
  
  .results-badge {
    font-size: 0.75rem;
    background: var(--color-primary-surface);
    color: var(--color-primary);
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    border: 1px solid var(--color-primary);
  }
  
  .endpoint-items { 
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 0.25rem;
    min-height: 0; /* Important: allows flex child to shrink below content size */
    height: 0; /* Force height to be calculated by flex */
    display: flex; 
    flex-direction: column; 
    gap: 0px; 
  }
  
  .empty-state { 
    display: flex; 
    flex-direction: column; 
    align-items: center; 
    justify-content: center; 
    padding: 2rem; 
    text-align: center; 
    height: 200px; 
    margin-top: 2rem;
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
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
</style>
