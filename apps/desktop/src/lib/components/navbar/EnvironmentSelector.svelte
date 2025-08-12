<script lang="ts">
  import { endpointStore } from '../../stores/endpoints.svelte.js';

  // Available environments - in real app this could come from project config
  const environments = [
    { id: 'dev', name: 'Development', baseUrl: 'http://localhost:3000' },
    { id: 'staging', name: 'Staging', baseUrl: 'https://api-staging.example.com' },
    { id: 'prod', name: 'Production', baseUrl: 'https://api.example.com' }
  ];

  let currentEnvironment = $state(environments[0]); // Default to dev
  let isOpen = $state(false);

  function selectEnvironment(env: typeof environments[0]) {
    currentEnvironment = env;
    isOpen = false;
    // Update the store with the new environment
    endpointStore.currentEnvironment = env;
  }

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  // Close dropdown when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as Element;
    if (!target.closest('.environment-selector')) {
      isOpen = false;
    }
  }

  // Add global click listener
  $effect(() => {
    if (isOpen) {
      document.addEventListener('click', handleClickOutside);
      return () => document.removeEventListener('click', handleClickOutside);
    }
  });
</script>

<div class="environment-selector">
  <button 
    class="environment-button" 
    onclick={toggleDropdown}
    class:open={isOpen}
  >
    <span class="environment-icon">🌍</span>
    <span class="environment-name">{currentEnvironment.name}</span>
    <span class="chevron" class:rotated={isOpen}>▼</span>
  </button>

  {#if isOpen}
    <div class="environment-dropdown">
      {#each environments as env}
        <button 
          class="environment-option"
          class:selected={env.id === currentEnvironment.id}
          onclick={() => selectEnvironment(env)}
        >
          <div class="environment-option-content">
            <div class="environment-option-name">{env.name}</div>
            <div class="environment-option-url">{env.baseUrl}</div>
          </div>
          {#if env.id === currentEnvironment.id}
            <span class="checkmark">✓</span>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .environment-selector {
    position: relative;
  }

  .environment-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: hsl(0 0% 9%);
    border: 1px solid hsl(0 0% 14.9%);
    border-radius: 6px;
    color: hsl(0 0% 98%);
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.9rem;
    min-width: 150px;
  }

  .environment-button:hover {
    background: hsl(0 0% 14.9%);
    border-color: hsl(0 0% 27.9%);
  }

  .environment-button.open {
    background: hsl(0 0% 14.9%);
    border-color: hsl(217.2 91.2% 59.8%);
  }

  .environment-icon {
    font-size: 1rem;
    flex-shrink: 0;
  }

  .environment-name {
    flex: 1;
    text-align: left;
    font-weight: 500;
  }

  .chevron {
    font-size: 0.7rem;
    transition: transform 0.2s;
    color: hsl(0 0% 71%);
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .environment-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: hsl(0 0% 9%);
    border: 1px solid hsl(0 0% 14.9%);
    border-radius: 6px;
    box-shadow: 
      0 10px 15px -3px rgb(0 0 0 / 0.1),
      0 4px 6px -4px rgb(0 0 0 / 0.1);
    z-index: 50;
    margin-top: 0.25rem;
    overflow: hidden;
  }

  .environment-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.75rem;
    background: transparent;
    border: none;
    color: hsl(0 0% 98%);
    cursor: pointer;
    transition: background-color 0.2s;
    text-align: left;
  }

  .environment-option:hover {
    background: hsl(0 0% 14.9%);
  }

  .environment-option.selected {
    background: hsl(217.2 91.2% 59.8% / 0.1);
  }

  .environment-option-content {
    flex: 1;
  }

  .environment-option-name {
    font-weight: 500;
    margin-bottom: 0.125rem;
  }

  .environment-option-url {
    font-size: 0.8rem;
    color: hsl(0 0% 71%);
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', monospace;
  }

  .checkmark {
    color: hsl(217.2 91.2% 59.8%);
    font-weight: bold;
  }
</style>
