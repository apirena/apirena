import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Endpoint, HttpResponse, RequestConfig, ProjectState, FileChangeEvent, EndpointManifest, Environment } from '../types';

function isTauri() {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ !== undefined;
}

function normalizeMethod(method: string | undefined): Endpoint['method'] {
  const m = (method ?? '').toString().toLowerCase();
  switch (m) {
    case 'get': return 'Get';
    case 'post': return 'Post';
    case 'put': return 'Put';
    case 'delete': return 'Delete';
    case 'patch': return 'Patch';
    case 'options': return 'Options';
    case 'head': return 'Head';
    default: return 'Get';
  }
}

function sanitizeEndpoint(e: Partial<Endpoint> & { method?: string; path?: string; handler?: string; line?: number; column?: number }): Endpoint {
  return {
    method: normalizeMethod(e.method as any),
    path: (e.path ?? '/').toString(),
    handler: (e.handler ?? (e as any).file ?? '').toString(),
    line: typeof e.line === 'number' ? e.line : 0,
    column: typeof e.column === 'number' ? e.column : 0,
    documentation: (e as any).documentation,
  };
}

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

  // Environment management
  currentEnvironment = $state<Environment>({
    id: 'dev',
    name: 'Development',
    baseUrl: 'http://localhost:3000'
  });

  lastResponse = $state<HttpResponse | null>(null);
  isLoading = $state<boolean>(false);
  error = $state<string | null>(null);

  constructor() {
    // Listen for real-time endpoint updates from Tauri
    this.setupEventListeners();
  }

  async loadFromFilesystem(projectPath: string) {
    this.projectState.path = projectPath;
    if (!isTauri()) {
      this.error = 'Tauri environment required. Run the desktop app (pnpm tauri dev).';
      return;
    }
    try {
      const content = await invoke<string | null>('read_manifest', { path: projectPath });
      if (content) {
        const manifest: EndpointManifest = JSON.parse(content);
        // Convert manifest entries to Endpoint shape for UI list
        this.projectState.endpoints = manifest.endpoints.map(e => sanitizeEndpoint({
          method: e.method as any,
          path: e.path,
          handler: (e as any).file,
          line: e.line,
          column: 0,
        }));
      }
    } catch (e) {
      console.warn('No manifest found yet, will generate on discovery.');
    }
  }

  async discoverEndpoints(projectPath: string) {
    if (!isTauri()) {
      this.error = 'Tauri environment required. Run the desktop app (pnpm tauri dev).';
      return;
    }
    try {
      this.isLoading = true;
      this.error = null;
      
      const endpointsRaw: any[] = await invoke('discover_endpoints', { 
        path: projectPath 
      });
      
      const endpoints = endpointsRaw.map(e => sanitizeEndpoint(e));
      this.projectState.endpoints = endpoints;
      this.projectState.path = projectPath;
      
      // Select first endpoint by default
      if (endpoints.length > 0 && !this.selectedEndpoint) {
        this.selectEndpoint(endpoints[0]);
      }
    } catch (err) {
      this.error = (err as string) ?? 'Failed to discover endpoints';
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
    if (!isTauri()) {
      this.error = 'Tauri environment required. Run the desktop app (pnpm tauri dev).';
      return;
    }

    try {
      const watchId: string = await invoke('start_watching', {
        path: this.projectState.path
      });
      
      this.projectState.watchId = watchId;
      this.projectState.isWatching = true;
    } catch (err) {
      this.error = (err as string) ?? 'Failed to start watching';
      console.error('Failed to start watching:', err);
    }
  }

  async stopWatching() {
    if (!this.projectState.watchId) return;
    if (!isTauri()) return;

    try {
      await invoke('stop_watching', {
        watchId: this.projectState.watchId
      });
      
      this.projectState.isWatching = false;
      this.projectState.watchId = undefined;
    } catch (err) {
      this.error = (err as string) ?? 'Failed to stop watching';
      console.error('Failed to stop watching:', err);
    }
  }

  clearProject() {
    this.projectState.path = '';
    this.projectState.endpoints = [];
    this.projectState.isWatching = false;
    this.projectState.watchId = undefined;
    this.selectedEndpoint = null;
    this.requestConfig = {
      endpoint: {} as Endpoint,
      params: {},
      headers: { 'Content-Type': 'application/json' },
      body: ''
    };
    this.lastResponse = null;
    this.error = null;
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

  setEnvironment(environment: Environment) {
    this.currentEnvironment = environment;
  }

  async sendRequest() {
    if (!this.selectedEndpoint) {
      this.error = 'No endpoint selected';
      return;
    }
    if (!isTauri()) {
      this.error = 'Tauri environment required. Run the desktop app (pnpm tauri dev).';
      return;
    }

    try {
      this.isLoading = true;
      this.error = null;

      const response: HttpResponse = await invoke('send_request', {
        endpoint: this.selectedEndpoint,
        params: this.requestConfig.params,
        headers: this.requestConfig.headers,
        body: this.requestConfig.body || null,
        baseUrl: this.currentEnvironment.baseUrl
      });

      this.lastResponse = response;
    } catch (err) {
      const msg = typeof err === 'string' ? err : (err as any)?.message ?? 'Request failed';
      // Surface errors in the Response panel
      this.lastResponse = {
        status: 0,
        headers: { 'content-type': 'text/plain' },
        body: String(msg),
        duration_ms: 0,
      };
      // Also keep error text if needed in the builder
      this.error = String(msg);
      console.error('Request failed:', err);
    } finally {
      this.isLoading = false;
    }
  }

  private async setupEventListeners() {
    if (!isTauri()) return;

    // Listen for file change events
    await listen<FileChangeEvent>('file-changed', (event) => {
      console.log('File changed:', event.payload);
    });

    // Listen for endpoint updates
    await listen<Endpoint[]>('endpoints-updated', (event) => {
      const updated = event.payload.map(e => sanitizeEndpoint(e as any));
      this.projectState.endpoints = updated;
      
      // Update selected endpoint if it still exists
      if (this.selectedEndpoint) {
        const updatedEndpoint = updated.find(ep => 
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
      const method = normalizeMethod(endpoint.method as any);
      if (!grouped[method]) {
        grouped[method] = [];
      }
      grouped[method].push(endpoint);
    }
    
    return grouped;
  }
}

// Export singleton instance
export const endpointStore = new EndpointStore();
