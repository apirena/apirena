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

export interface EndpointEntry {
  id: string;
  method: string;
  path: string;
  file: string;
  line: number;
  framework: string;
  last_seen: string;
}

export interface EndpointManifest {
  version: string;
  last_updated: string;
  endpoints: EndpointEntry[];
  statistics: { total: number; by_method: Record<string, number> };
}

export interface Environment {
  id: string;
  name: string;
  baseUrl: string;
}

export interface ProjectConfig {
  version: string;
  baseUrl: string;
  environments?: Record<string, { baseUrl: string }>;
  watch: { include: string[]; exclude: string[] };
  parser?: { incrementalEnabled: boolean; parallelism: number };
}
