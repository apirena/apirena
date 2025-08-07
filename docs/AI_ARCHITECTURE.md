# AI Architecture for Reqsmith

## Overview

The AI layer enhances endpoint discovery with intelligent parameter analysis, while maintaining performance through smart caching and minimal processing.

## Core Design Principles

1. **Cache First**: AI runs once per unique endpoint signature
2. **Local First**: Prefer local models for privacy and speed
3. **Progressive Enhancement**: AI improves experience but isn't required
4. **User Override**: Manual inputs always take precedence

## Architecture

```
Endpoint Detected → Signature Generated → Cache Check → AI Analysis → Store Result
                            ↓                ↓              ↓            ↓
                     Hash(method+path)    If miss      Analyze once   Cache forever*
                                                                    
                                                    *Until signature changes
```

## Caching Strategy

### Signature Generation
```typescript
function generateEndpointSignature(endpoint: Endpoint): string {
  return crypto.hash({
    method: endpoint.method,
    path: endpoint.path,
    paramNames: endpoint.params.map(p => p.name),
    middlewares: endpoint.middlewares,
    fileHash: endpoint.sourceFile.contentHash
  });
}
```

### Cache Structure
```typescript
interface AICache {
  signature: string;
  timestamp: number;
  analysis: {
    parameters: ParameterAnalysis[];
    security: SecurityRequirements;
    examples: ExampleValues;
    documentation: ExtractedDocs;
  };
  userOverrides?: {
    // User's custom values preserved
    [key: string]: any;
  };
}
```

## AI Analysis Pipeline

### 1. Context Extraction
```typescript
// Gather context around endpoint
const context = {
  functionSignature: getFunctionSignature(endpoint),
  surroundingCode: getCodeContext(endpoint, lines = 20),
  fileImports: getImports(endpoint.file),
  middlewares: getMiddlewares(endpoint),
  comments: getNearbyComments(endpoint)
};
```

### 2. Intelligent Analysis
```typescript
// Single AI call per endpoint
const analysis = await ai.analyze({
  endpoint,
  context,
  prompt: ENDPOINT_ANALYSIS_PROMPT
});

// Returns structured data
{
  parameters: [
    {
      name: 'userId',
      type: 'string',
      required: true,
      format: 'uuid',
      example: 'usr_123abc',
      description: 'User identifier'
    }
  ],
  security: {
    type: 'bearer',
    required: true,
    headerName: 'Authorization'
  }
}
```

### 3. Cache Storage
```typescript
// Store in local cache
await cache.set(signature, {
  signature,
  timestamp: Date.now(),
  analysis,
  ttl: Infinity // Never expires unless signature changes
});
```

## Prompt Engineering

### Endpoint Analysis Prompt
```
You are analyzing an API endpoint. Extract:

1. Parameters (path/query/body):
   - Name, type, required/optional
   - Valid formats/patterns
   - Example values

2. Security requirements:
   - Auth type (bearer/basic/api-key)
   - Required headers
   
3. Request/response format:
   - Content types
   - Body structure

Context:
<function_signature>
<surrounding_code>
<middleware_chain>

Respond in JSON format...
```

## Performance Optimization

### Lazy Loading
- AI only runs when endpoint is viewed in playground
- Background queue for non-blocking analysis
- Progressive UI updates as analysis completes

### Batch Processing
```typescript
// Batch multiple endpoints for efficiency
const batchAnalysis = await ai.batchAnalyze(
  newEndpoints.map(e => ({
    endpoint: e,
    context: getContext(e)
  }))
);
```

### Local Model Integration
```typescript
// Prioritize local models for speed
const providers = [
  new OllamaProvider({ model: 'codellama' }),
  new OpenAIProvider({ apiKey: process.env.OPENAI_KEY })
];

// Fallback chain
for (const provider of providers) {
  try {
    return await provider.analyze(prompt);
  } catch (e) {
    continue; // Try next provider
  }
}
```

## Cache Invalidation

### When to Invalidate
1. Endpoint signature changes (method, path, params)
2. Function implementation significantly changes
3. Middleware chain modifications
4. User explicitly refreshes

### Partial Invalidation
```typescript
// Only re-analyze what changed
if (onlyMiddlewareChanged(oldEndpoint, newEndpoint)) {
  // Just re-analyze security requirements
  const securityAnalysis = await ai.analyzePartial({
    type: 'security',
    endpoint: newEndpoint
  });
  
  // Merge with existing cache
  cache.update(signature, { security: securityAnalysis });
}
```

## User Experience

### Progressive Enhancement
```typescript
// Show endpoint immediately
ui.showEndpoint(endpoint);

// Check cache
const cached = await cache.get(endpoint.signature);
if (cached) {
  ui.enhanceWithAI(cached.analysis);
  return;
}

// Queue for analysis
analysisQueue.add(endpoint, priority = 'low');

// Update UI when ready
analysisQueue.on('complete', (endpoint, analysis) => {
  ui.enhanceWithAI(analysis);
});
```

### Preserving User State
```typescript
// User's values always win
const finalValues = {
  ...aiSuggestions,
  ...cachedUserValues,
  ...currentUserInput
};
```

## Error Handling

```typescript
// Graceful degradation
try {
  const analysis = await ai.analyze(endpoint);
  cache.store(analysis);
} catch (error) {
  logger.warn('AI analysis failed, using fallbacks', error);
  
  // Use rule-based fallbacks
  const fallback = generateBasicAnalysis(endpoint);
  cache.store(fallback, { provisional: true });
}
```

## Privacy & Security

1. **Local First**: Prefer on-device models
2. **Opt-in Cloud**: Explicit consent for cloud AI
3. **No Code Transmission**: Only send minimal context
4. **Secure Storage**: Encrypted cache on disk
