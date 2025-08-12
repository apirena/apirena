<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { endpointStore } from '../../stores/endpoints.svelte.js';
  import EnvironmentSelector from './EnvironmentSelector.svelte';
  import { Eye } from '@lucide/svelte';

  function isTauri() {
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

  // Get project display name
  const projectDisplayName = $derived(() => {
    if (!projectState.path) return 'No Project Selected';
    return projectState.path.split('/').pop() || projectState.path;
  });
</script>

<nav class="top-navbar">
  <div class="navbar-left">
    <!-- Project Selector -->
    <button 
      class="project-button" 
      onclick={selectProject}
      disabled={isLoading || !isTauri()}
      title={projectState.path || 'Select a project folder'}
    >
      <span class="project-icon">📁</span>
      <span class="project-name">{projectDisplayName}</span>
      {#if isLoading}
        <div class="spinner"></div>
      {/if}
    </button>

    <!-- Watch Controls -->
    {#if projectState.path}
      <div class="watch-controls">
        <button 
          class="watch-button"
          class:watching={projectState.isWatching}
          onclick={toggleWatching}
          disabled={isLoading || !isTauri()}
        >
          {#if projectState.isWatching}
            <span class="watch-icon">⏹️</span>
            Stop Watching
          {:else}
            <span class="watch-icon">▶️</span>
            Start Watching
          {/if}
        </button>

        <div class="endpoint-count">
          {projectState.endpoints.length} endpoints
        </div>
      </div>
    {/if}
  </div>

  <div class="navbar-right">
    <!-- Environment selector and visibility action -->
    <EnvironmentSelector />
    <button class="watch-button" title="Toggle visibility" aria-label="Toggle visibility">
      <Eye class="size-4" />
    </button>
  </div>
</nav>

<style>
  .top-navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: hsl(0 0% 2%);
    border-bottom: 1px solid hsl(0 0% 14.9%);
    min-height: 60px;
    gap: 1rem;
  }

  .navbar-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .navbar-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .project-button {
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
    max-width: 250px;
  }

  .project-button:hover {
    background: hsl(0 0% 14.9%);
    border-color: hsl(0 0% 27.9%);
  }

  .project-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .project-icon {
    font-size: 1rem;
    flex-shrink: 0;
  }

  .project-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-weight: 500;
  }

  .watch-controls {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .watch-button {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.625rem;
    background: transparent;
    border: 1px solid hsl(0 0% 27.9%);
    border-radius: 4px;
    color: hsl(0 0% 71%);
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.8rem;
  }

  .watch-button:hover {
    background: hsl(0 0% 14.9%);
    color: hsl(0 0% 98%);
  }

  .watch-button.watching {
    background: hsl(142.1 76.2% 36.3%);
    color: hsl(355.7 100% 97.3%);
    border-color: hsl(142.1 76.2% 36.3%);
  }

  .watch-button.watching:hover {
    background: hsl(142.1 70% 45%);
  }

  .watch-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .watch-icon {
    font-size: 0.8rem;
  }

  .endpoint-count {
    font-size: 0.8rem;
    color: hsl(0 0% 71%);
    padding: 0.25rem 0;
  }

  .spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid hsl(0 0% 27.9%);
    border-top: 2px solid hsl(0 0% 98%);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
