export type HttpMethod = 'Get' | 'Post' | 'Put' | 'Delete' | 'Patch' | 'Options' | 'Head';

export interface Endpoint {
  method: HttpMethod;
  path: string;
  handler: string;
  line: number;
  column: number;
  documentation?: string;
}

export interface HttpResponse {
  status: number;
  headers: Record<string, string>;
  body: string;
  duration_ms: number;
}

export interface FileChangeEvent {
  source: ChangeSource;
  diffs: any[];
  timestamp: string;
}

export interface ChangeSource {
  // This will be expanded based on the Rust enum structure
  [key: string]: any;
}

export interface RequestConfig {
  endpoint: Endpoint;
  params: Record<string, string>;
  headers: Record<string, string>;
  body?: string;
}

export interface ProjectState {
  path: string;
  endpoints: Endpoint[];
  isWatching: boolean;
  watchId?: string;
}
