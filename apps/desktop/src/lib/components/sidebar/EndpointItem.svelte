<script lang="ts">
  import type { Endpoint } from '../../types';

  interface Props {
    endpoint: Endpoint;
    isSelected: boolean;
    onclick: () => void;
  }

  let { endpoint, isSelected, onclick }: Props = $props();

  const methodColors: Record<string, string> = {
    'Get': '#10B981',      // Green
    'Post': '#F59E0B',     // Amber  
    'Put': '#3B82F6',      // Blue
    'Delete': '#EF4444',   // Red
    'Patch': '#8B5CF6',    // Purple
    'Options': '#6B7280',  // Gray
    'Head': '#6B7280'      // Gray
  };

  // Extract path segments for better readability
  const pathSegments = $derived(() => {
    const segments = endpoint.path.split('/').filter(Boolean);
    return segments;
  });

  // Check if path has dynamic segments (contains : or {})
  const hasDynamicSegments = $derived(() => {
    return endpoint.path.includes(':') || endpoint.path.includes('{');
  });
</script>

<div 
  class="endpoint-item" 
  class:selected={isSelected}
  onclick={onclick}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && onclick()}
>
  <div class="endpoint-header">
    <span 
      class="method-badge"
      style="background-color: {methodColors[endpoint.method]};"
    >
      {endpoint.method}
    </span>
    <div class="endpoint-path">
      {#if pathSegments.length === 0}
        <span class="path-segment">/</span>
      {:else}
        {#each pathSegments() as segment, i}
          <span class="path-separator">/</span>
          <span 
            class="path-segment"
            class:dynamic={(segment as string).startsWith(':') || ((segment as string).startsWith('{') && (segment as string).endsWith('}'))}
          >
            {segment}
          </span>
        {/each}
      {/if}
    </div>
  </div>

  {#if endpoint.documentation}
    <div class="endpoint-doc">
      {endpoint.documentation}
    </div>
  {/if}

  <div class="endpoint-meta">
    <span class="file-location">
      📍 Line {endpoint.line}:{endpoint.column}
    </span>
      {#if hasDynamicSegments()}
      <span class="dynamic-indicator" title="Has dynamic path segments">
        🔗 Dynamic
      </span>
    {/if}
  </div>
</div>

<style>
  .endpoint-item {
    padding: 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    border: 1px solid transparent;
    background: var(--color-surface);
  }

  .endpoint-item:hover {
    background: var(--color-surface-hover);
    border-color: var(--color-border-hover);
  }

  .endpoint-item.selected {
    background: var(--color-primary-surface);
    border-color: var(--color-primary);
    box-shadow: 0 0 0 1px var(--color-primary-alpha);
  }

  .endpoint-item:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: -2px;
  }

  .endpoint-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .method-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    color: var(--color-primary-foreground);
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.025em;
    min-width: 3rem;
    text-align: center;
  }

  .endpoint-path {
    flex: 1;
    display: flex;
    align-items: center;
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', monospace;
    font-size: 0.85rem;
    line-height: 1.2;
    overflow: hidden;
  }

  .path-separator {
    color: var(--color-text-tertiary);
    margin: 0 0.1rem;
  }

  .path-segment {
    color: var(--color-text-primary);
    white-space: nowrap;
  }

  .path-segment.dynamic {
    background: var(--color-warning-surface);
    color: var(--color-warning);
    padding: 0.125rem 0.25rem;
    border-radius: 3px;
    font-weight: 500;
  }

  .endpoint-doc {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
    margin-bottom: 0.5rem;
    line-height: 1.3;
    font-style: italic;
  }

  .endpoint-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.7rem;
    color: var(--color-text-tertiary);
  }

  .file-location {
    opacity: 0.8;
  }

  .dynamic-indicator {
    background: var(--color-warning-surface);
    color: var(--color-warning);
    padding: 0.125rem 0.25rem;
    border-radius: 3px;
    font-weight: 500;
  }
</style>
