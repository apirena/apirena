<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { endpointStore } from '../../stores/endpoints.svelte.js';

  function isTauri() {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ !== undefined;
  }

  // Access store properties directly
  let projectState = $derived(endpointStore.projectState);
  let isLoading = $derived(endpointStore.isLoading);

  async function selectProject() {
    if (!isTauri()) {
      console.warn('Tauri environment required. Run the desktop app (pnpm tauri dev).');
      return;
    }
    try {
      // Stop watching current project if active
      if (projectState.isWatching) {
        await endpointStore.stopWatching();
      }
      
      // Use Tauri command to select project folder
      const selectedPath = await invoke('select_project_folder');
      if (selectedPath) {
        // Clear previous project state
        endpointStore.clearProject();
        
        // Load from filesystem manifest first for instant UI
        await endpointStore.loadFromFilesystem(selectedPath as string);
        // Then run discovery to refresh and persist
        await endpointStore.discoverEndpoints(selectedPath as string);
      }
    } catch (error) {
      console.error('Failed to select project:', error);
    }
  }

  async function toggleWatching() {
    if (!isTauri()) return;
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
        disabled={isLoading || !isTauri()}
        title={!isTauri() ? 'Run the desktop app with Tauri to enable watching' : ''}
      >
        {projectState.isWatching ? '⏹️ Stop Watching' : '▶️ Start Watching'}
      </button>
    {/if}
  </div>

  {#if !isTauri()}
    <div class="tauri-hint">
      ⚠️ Project selection requires Tauri. Run the desktop app (pnpm tauri dev) instead of Vite-only.
    </div>
  {/if}

  {#if projectState.path}
    <div class="project-info">
      <div class="project-header">
        <div class="project-path" title={projectState.path}>
          📁 {projectState.path.split('/').pop()}
        </div>
        <button 
          class="reselect-btn" 
          onclick={selectProject} 
          disabled={isLoading || !isTauri()}
          title="Select different project folder"
        >
          🔄
        </button>
      </div>
      <div class="endpoint-count">
        {projectState.endpoints.length} endpoints found
      </div>
      <div class="project-actions">
        <button 
          class="change-project-btn" 
          onclick={selectProject} 
          disabled={isLoading || !isTauri()}
        >
          📂 Change Project
        </button>
      </div>
    </div>
  {:else}
    <button class="select-project-btn" onclick={selectProject} disabled={isLoading || !isTauri()} title={!isTauri() ? 'Run the desktop app with Tauri to select a project' : ''}>
      {isLoading ? 'Loading...' : '📁 Select Project Folder'}
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

  .tauri-hint {
    margin: 0.5rem 0 0.75rem 0;
    padding: 0.5rem 0.75rem;
    font-size: 0.85rem;
    background: var(--color-warning-surface);
    color: var(--color-warning);
    border: 1px dashed var(--color-warning);
    border-radius: 6px;
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
    gap: 0.5rem;
  }

  .project-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
  }

  .project-path {
    font-family: monospace;
    font-size: 0.9rem;
    color: var(--color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .reselect-btn {
    padding: 0.2rem 0.4rem;
    font-size: 0.8rem;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .reselect-btn:hover {
    background: var(--color-surface-hover);
    color: var(--color-primary);
  }

  .reselect-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .endpoint-count {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
  }

  .project-actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .change-project-btn {
    padding: 0.4rem 0.75rem;
    font-size: 0.8rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    flex: 1;
  }

  .change-project-btn:hover {
    background: var(--color-surface-hover);
    color: var(--color-primary);
    border-color: var(--color-primary);
  }

  .change-project-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
