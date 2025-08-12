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
</script>

<div 
  class="endpoint-item" 
  class:selected={isSelected}
  onclick={onclick}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && onclick()}
>
  <span 
    class="method-badge"
    style="background-color: {methodColors[endpoint.method]};"
  >
    {endpoint.method.toUpperCase()}
  </span>
  <span class="endpoint-path">{endpoint.path}</span>
</div>

<style>
  .endpoint-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
    border: 1px solid transparent;
    background: var(--color-surface);
    min-height: auto;
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

  .method-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.125rem 0.375rem;
    border-radius: 3px;
    color: white;
    font-size: 0.65rem;
    font-weight: 600;
    letter-spacing: 0.025em;
    min-width: 2.75rem;
    text-align: center;
    flex-shrink: 0;
    line-height: 1.2;
  }

  .endpoint-path {
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', monospace;
    font-size: 0.8rem;
    color: var(--color-text-primary);
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    line-height: 1.2;
  }
</style>
