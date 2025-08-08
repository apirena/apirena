import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Endpoint, HttpResponse, RequestConfig, ProjectState, FileChangeEvent } from '../types';

class EndpointStore {
  // Svelte 5 runes for reactive state
  projectState = $state<ProjectState>({
    path: '',
    endpoints: [],
    isWatching: false,
    watchId: undefined
  });

  selectedEndpoint = $state<Endpoint | null>(null);
  requestConfig = $state<RequestConfig>({
    endpoint: {} as Endpoint,
    params: {},
    headers: { 'Content-Type': 'application/json' },
    body: ''
  });

  lastResponse = $state<HttpResponse | null>(null);
  isLoading = $state<boolean>(false);
  error = $state<string | null>(null);

  constructor() {
    // Listen for real-time endpoint updates from Tauri
    this.setupEventListeners();
  }

  async discoverEndpoints(projectPath: string) {
    try {
      this.isLoading = true;
      this.error = null;
      
      const endpoints: Endpoint[] = await invoke('discover_endpoints', { 
        path: projectPath 
      });
      
      this.projectState.endpoints = endpoints;
      this.projectState.path = projectPath;
      
      // Select first endpoint by default
      if (endpoints.length > 0 && !this.selectedEndpoint) {
        this.selectEndpoint(endpoints[0]);
      }
    } catch (err) {
      this.error = err as string;
      console.error('Failed to discover endpoints:', err);
    } finally {
      this.isLoading = false;
    }
  }

  async startWatching() {
    if (!this.projectState.path) {
      this.error = 'No project path selected';
      return;
    }

    try {
      const watchId: string = await invoke('start_watching', {
        path: this.projectState.path
      });
      
      this.projectState.watchId = watchId;
      this.projectState.isWatching = true;
    } catch (err) {
      this.error = err as string;
      console.error('Failed to start watching:', err);
    }
  }

  async stopWatching() {
    if (!this.projectState.watchId) return;

    try {
      await invoke('stop_watching', {
        watchId: this.projectState.watchId
      });
      
      this.projectState.isWatching = false;
      this.projectState.watchId = undefined;
    } catch (err) {
      this.error = err as string;
      console.error('Failed to stop watching:', err);
    }
  }

  selectEndpoint(endpoint: Endpoint) {
    this.selectedEndpoint = endpoint;
    this.requestConfig.endpoint = endpoint;
    // Reset form state
    this.requestConfig.params = {};
    this.requestConfig.body = '';
    this.lastResponse = null;
  }

  updateRequestParam(key: string, value: string) {
    this.requestConfig.params[key] = value;
  }

  updateRequestHeader(key: string, value: string) {
    this.requestConfig.headers[key] = value;
  }

  updateRequestBody(body: string) {
    this.requestConfig.body = body;
  }

  async sendRequest() {
    if (!this.selectedEndpoint) {
      this.error = 'No endpoint selected';
      return;
    }

    try {
      this.isLoading = true;
      this.error = null;

      const response: HttpResponse = await invoke('send_request', {
        endpoint: this.selectedEndpoint,
        params: this.requestConfig.params,
        headers: this.requestConfig.headers,
        body: this.requestConfig.body || null
      });

      this.lastResponse = response;
    } catch (err) {
      this.error = err as string;
      console.error('Request failed:', err);
    } finally {
      this.isLoading = false;
    }
  }

  private async setupEventListeners() {
    // Listen for file change events
    await listen<FileChangeEvent>('file-changed', (event) => {
      console.log('File changed:', event.payload);
    });

    // Listen for endpoint updates
    await listen<Endpoint[]>('endpoints-updated', (event) => {
      this.projectState.endpoints = event.payload;
      
      // Update selected endpoint if it still exists
      if (this.selectedEndpoint) {
        const updatedEndpoint = event.payload.find(ep => 
          ep.path === this.selectedEndpoint?.path && 
          ep.method === this.selectedEndpoint?.method
        );
        
        if (updatedEndpoint) {
          this.selectedEndpoint = updatedEndpoint;
          this.requestConfig.endpoint = updatedEndpoint;
        }
      }
    });
  }

  // Computed properties (Svelte 5 runes style)
  get httpMethods() {
    return ['Get', 'Post', 'Put', 'Delete', 'Patch', 'Options', 'Head'];
  }

  get endpointsByMethod() {
    const grouped: Record<string, Endpoint[]> = {};
    
    for (const endpoint of this.projectState.endpoints) {
      if (!grouped[endpoint.method]) {
        grouped[endpoint.method] = [];
      }
      grouped[endpoint.method].push(endpoint);
    }
    
    return grouped;
  }
}

// Export singleton instance
export const endpointStore = new EndpointStore();
