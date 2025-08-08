<script lang="ts">
  import type { HttpMethod } from '../../types';

  interface Props {
    method: HttpMethod;
    readonly?: boolean;
    onMethodChange?: (method: HttpMethod) => void;
  }

  let { method, readonly = false, onMethodChange }: Props = $props();

  const methodColors: Record<HttpMethod, string> = {
    'Get': '#10B981',      // Green
    'Post': '#F59E0B',     // Amber  
    'Put': '#3B82F6',      // Blue
    'Delete': '#EF4444',   // Red
    'Patch': '#8B5CF6',    // Purple
    'Options': '#6B7280',  // Gray
    'Head': '#6B7280'      // Gray
  };

  const methods: HttpMethod[] = ['Get', 'Post', 'Put', 'Delete', 'Patch', 'Options', 'Head'];
</script>

{#if readonly}
  <span 
    class="method-badge readonly"
    style="background-color: {methodColors[method]};"
  >
    {method.toUpperCase()}
  </span>
{:else}
  <select 
    class="method-select"
    style="border-color: {methodColors[method]}; color: {methodColors[method]};"
    value={method}
    onchange={(e) => onMethodChange?.(e.currentTarget.value as HttpMethod)}
  >
    {#each methods as methodOption}
      <option value={methodOption} style="color: {methodColors[methodOption]};">
        {methodOption.toUpperCase()}
      </option>
    {/each}
  </select>
{/if}

<style>
  .method-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.375rem 0.75rem;
    border-radius: 6px;
    color: white;
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.025em;
    min-width: 4rem;
    text-align: center;
  }

  .method-badge.readonly {
    border: 2px solid currentColor;
    background: transparent !important;
    color: var(--method-color);
  }

  .method-select {
    padding: 0.375rem 0.75rem;
    border: 2px solid;
    border-radius: 6px;
    background: var(--color-surface);
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.025em;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 5rem;
  }

  .method-select:hover {
    background: var(--color-surface-hover);
  }

  .method-select:focus {
    outline: none;
    box-shadow: 0 0 0 2px rgba(var(--color-primary-rgb), 0.2);
  }

  .method-select option {
    background: var(--color-surface);
    padding: 0.5rem;
  }
</style>
