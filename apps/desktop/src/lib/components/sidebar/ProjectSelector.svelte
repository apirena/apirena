<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { endpointStore } from '../../stores/endpoints.svelte.js';

  // Access store properties directly
  let projectState = $derived(endpointStore.projectState);
  let isLoading = $derived(endpointStore.isLoading);

  async function selectProject() {
    try {
      // Use Tauri command to select project folder
      const selectedPath = await invoke('select_project_folder');
      if (selectedPath) {
        await endpointStore.discoverEndpoints(selectedPath as string);
      }
    } catch (error) {
      console.error('Failed to select project:', error);
    }
  }

  async function toggleWatching() {
    if (projectState.isWatching) {
      await endpointStore.stopWatching();
    } else {
      await endpointStore.startWatching();
    }
  }
</script>

<div class="project-selector">
  <div class="header">
    <h2>Project</h2>
    {#if projectState.path}
      <button 
        class="watch-toggle" 
        class:watching={projectState.isWatching}
        onclick={toggleWatching}
        disabled={isLoading}
      >
        {projectState.isWatching ? '⏹️ Stop Watching' : '▶️ Start Watching'}
      </button>
    {/if}
  </div>

  {#if projectState.path}
    <div class="project-info">
      <div class="project-path" title={projectState.path}>
        📁 {projectState.path.split('/').pop()}
      </div>
      <div class="endpoint-count">
        {projectState.endpoints.length} endpoints found
      </div>
    </div>
  {:else}
    <button class="select-project-btn" onclick={selectProject} disabled={isLoading}>
      {isLoading ? 'Loading...' : '📁 Select Project'}
    </button>
  {/if}
</div>

<style>
  .project-selector {
    padding: 1rem;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .header h2 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .watch-toggle {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .watch-toggle:hover {
    background: var(--color-surface-hover);
  }

  .watch-toggle.watching {
    background: var(--color-success);
    color: var(--color-primary-foreground);
    border-color: var(--color-success);
  }

  .project-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .project-path {
    font-family: monospace;
    font-size: 0.9rem;
    color: var(--color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .endpoint-count {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
  }

  .select-project-btn {
    width: 100%;
    padding: 0.75rem;
    border: 2px dashed var(--color-border);
    border-radius: 8px;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.9rem;
  }

  .select-project-btn:hover {
    border-color: var(--color-primary);
    color: var(--color-primary);
  }

  .select-project-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
