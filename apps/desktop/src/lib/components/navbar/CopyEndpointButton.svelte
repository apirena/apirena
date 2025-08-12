<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { endpointStore } from '../../stores/endpoints.svelte.js';

  let selectedEndpoint = $derived(endpointStore.selectedEndpoint);
  let currentEnvironment = $derived(endpointStore.currentEnvironment);
  let requestConfig = $derived(endpointStore.requestConfig);

  function isTauri() {
    return typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ !== undefined;
  }

  async function copyEndpoint() {
    if (!selectedEndpoint || !currentEnvironment) return;

    try {
      // Build the full URL
      let url = currentEnvironment.baseUrl + selectedEndpoint.path;
      
      // Add query parameters if any
      const params = Object.entries(requestConfig.params)
        .filter(([_, value]) => value.trim() !== '')
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
        .join('&');
      
      if (params) {
        url += '?' + params;
      }

      // Format the final string
      const copyText = `${selectedEndpoint.method.toUpperCase()} ${url}`;

      // Try browser clipboard API first, then Tauri as fallback
      try {
        await navigator.clipboard.writeText(copyText);
      } catch {
        // Fallback to Tauri command
        if (isTauri()) {
          await invoke('copy_to_clipboard', { text: copyText });
        } else {
          throw new Error('Clipboard not available');
        }
      }

      // Show success feedback
      showCopyFeedback();
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
    }
  }

  let showSuccess = $state(false);

  function showCopyFeedback() {
    showSuccess = true;
    setTimeout(() => {
      showSuccess = false;
    }, 2000);
  }
</script>

<button 
  class="copy-button"
  onclick={copyEndpoint}
  disabled={!selectedEndpoint || !currentEnvironment}
  title="Copy endpoint URL to clipboard"
>
  {#if showSuccess}
    <span class="copy-icon success">✓</span>
    Copied!
  {:else}
    <span class="copy-icon">📋</span>
    Copy
  {/if}
</button>

<style>
  .copy-button {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.75rem;
    background: hsl(0 0% 9%);
    border: 1px solid hsl(0 0% 14.9%);
    border-radius: 6px;
    color: hsl(0 0% 98%);
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .copy-button:hover {
    background: hsl(0 0% 14.9%);
    border-color: hsl(0 0% 27.9%);
  }

  .copy-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .copy-button:active {
    transform: translateY(1px);
  }

  .copy-icon {
    font-size: 0.9rem;
  }

  .copy-icon.success {
    color: hsl(142.1 76.2% 36.3%);
  }
</style>
