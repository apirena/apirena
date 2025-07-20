# Hallwatch Product Specification

## Core Concept

Hallwatch is a **code-first API playground** that eliminates the gap between implementation and testing. By combining AST parsing with intelligent caching and AI assistance, we provide instant API testing without manual configuration.

## Key Principles

### 1. Code is Truth
- The implementation IS the specification
- No separate documentation to maintain
- No drift between code and tests

### 2. Zero Friction Testing
- Save file → Test API (no steps in between)
- No manual URL construction
- No parameter guessing

### 3. Intelligent, Not Intrusive
- AI enhances, doesn't replace developer knowledge
- Suggestions are cached and refined
- Your playground state is preserved

## Core User Journey

```
Developer writes endpoint → Hallwatch detects it → AI analyzes context → 
Playground ready → Developer tests → State preserved → Iterate quickly
```

## Technical Architecture

### Parser Library (`@hallwatch/parser`)
The heart of Hallwatch - a high-performance, multi-language endpoint detector.

**Capabilities:**
- AST-based endpoint discovery
- Framework-agnostic pattern matching
- <10ms parsing performance
- Incremental parsing on changes

**Supported Patterns:**
```javascript
// Express
app.get('/users/:id', handler)
router.post('/api/users', middleware, handler)

// Decorator-based
@Get('/users/:id')
@RequireAuth()
async getUser(id: string) {}

// Function-based
defineRoute('GET', '/users', getUserHandler)
```

### AI Enhancement Layer (`@hallwatch/ai`)
Intelligent parameter and context analysis, with smart caching.

**Features:**
- **Initial Analysis**: On first endpoint discovery
  - Parameter type inference
  - Required/optional detection
  - Security requirements extraction
  - Example value generation
  
- **Smart Caching**: Prevents redundant AI calls
  - Cache by endpoint signature
  - Invalidate on signature changes
  - Preserve user overrides

**Example Flow:**
```typescript
// Code detected
app.post('/users/:userId/documents', uploadDocument)

// AI analyzes ONCE and caches:
{
  params: { userId: { type: 'string', example: 'usr_123' } },
  body: { 
    file: { type: 'file', required: true },
    description: { type: 'string', required: false }
  },
  headers: {
    'Authorization': { required: true, example: 'Bearer ...' }
  }
}

// Future file saves use cached analysis unless signature changes
```

### Playground UI (`@hallwatch/desktop`)
A persistent, intelligent testing environment.

**Key Features:**
- **State Preservation**: Your test values persist across sessions
- **Environment Management**: Switch between local/staging/production
- **Smart Defaults**: AI suggestions as starting points
- **Request History**: Recent tests for quick replay

## Configuration Philosophy

### Zero Config by Default
Hallwatch works immediately with standard patterns:
- No annotations required
- No special comments needed
- No configuration files necessary

### Progressive Enhancement
Optional configuration for advanced needs:

```javascript
// .hallwatch/config.js - OPTIONAL
export default {
  // Custom file watching
  watch: {
    include: ['src/**/*.js', 'api/**/*.py'],
    exclude: ['tests/**', 'node_modules/**']
  },
  
  // AI behavior tuning
  ai: {
    cacheStrategy: 'aggressive', // or 'conservative'
    providers: ['local', 'openai'], // fallback chain
    analysisDepth: 'full' // or 'basic'
  },
  
  // Parameter overrides
  defaults: {
    userId: () => `user_${Date.now()}`,
    apiKey: () => process.env.TEST_API_KEY
  }
}
```

## Differentiation Strategy

### vs Postman/Insomnia
- **They**: Manual collection management
- **We**: Automatic discovery from code
- **Advantage**: Never out of sync

### vs Swagger/OpenAPI
- **They**: Require annotations/specs
- **We**: Work with vanilla code
- **Advantage**: Zero adoption friction

### vs REST Client Extensions
- **They**: File-based, manual editing
- **We**: Visual playground, AI assistance
- **Advantage**: Faster iteration, smarter defaults

## Success Metrics

1. **Time to First Test**: <30 seconds from install
2. **Endpoint Detection Rate**: >95% on standard patterns
3. **AI Cache Hit Rate**: >90% on subsequent runs
4. **Performance**: <10ms parse, <100ms AI enhancement

## Future Expansion Ideas

_These are not core features but potential future additions:_

### Public API Monitoring
- Track external API changes
- Generate migration guides
- Alert on breaking changes

### Team Collaboration
- Shared playground sessions
- API change reviews
- Test scenario sharing

### Advanced Testing
- Load testing integration
- Contract testing
- Mock server generation

### IDE Integration
- VS Code extension
- IntelliJ plugin
- Inline test execution

### Protocol Expansion
- GraphQL support
- gRPC playground
- WebSocket testing

## Development Priorities

1. **Core Parser Excellence**: Rock-solid endpoint detection
2. **AI Caching System**: Smart, efficient context analysis
3. **Playground Polish**: Delightful testing experience
4. **Performance**: Sub-10ms always
5. **Framework Coverage**: Support top 20 frameworks
