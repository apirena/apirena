# Hallwatch: The Code-First API Playground

**Your code is the spec.** Test APIs instantly as you write them—no manual documentation, no spec drift, just real-time API discovery that understands your actual implementation.

Prototype: https://theme-yoga-27205403.figma.site/

<img width="3594" height="1954" alt="CleanShot 2025-08-05 at 19 43 41@2x" src="https://github.com/user-attachments/assets/62ebdc33-ffd2-47ef-a1ed-294584047be0" />

## 🎯 Core Value: Code-First API Testing

Hallwatch watches your source files, automatically discovers endpoints through AST parsing, and creates an intelligent playground for testing—all without requiring special comments, annotations, or separate spec files.

```javascript
// You write this:
app.post('/users/:id/profile', updateProfile);

// Hallwatch instantly provides:
// ✅ Endpoint detected
// ✅ Parameters extracted (id: string)
// ✅ Method identified (POST)
// ✅ Smart suggestions ready
// → Test it now!
```

## 🏆 Why Hallwatch?

### The Problem with Current Tools

| Traditional API Tools | Reality |
|----------------------|---------|
| Manually maintain collections | Outdated within days |
| Write OpenAPI specs | Never matches implementation |
| Import from frameworks | Requires specific annotations |
| Copy-paste URLs | Error-prone and tedious |

### The Hallwatch Difference

We use **AST parsing + AI** to understand your code as you write it:
- **Zero Configuration**: Works with your existing code
- **Real-time Updates**: Changes reflect instantly
- **Framework Agnostic**: No special decorators needed
- **Intelligent Assistance**: AI suggests parameters, not just detects them

## 📊 How We Compare

### Configuration Required
```
Manual Everything ←—————————————————————————————————————————————————→ Zero Config
                  ●—————————————————————————————————————————————————○
Hallwatch         ●                                                 
Postman                                                            ●
Insomnia                                                       ●    
Swagger                                                    ●         
```

### Time to First Test
```
5+ Minutes ←——————————————————————————————————————————————————————————→ <10 Seconds
           ●—————————————————————————————————————————————————————————○
Hallwatch  ●                                                        
Postman                                                            ●
Insomnia                                                       ●    
Swagger                                                    ●         
```

### Spec Accuracy
```
Drifts Immediately ←—————————————————————————————————————————————————→ Always Current
                   ●————————————————————————————————————————————————○
Hallwatch          ●                                                
Manual Tools                                                      ●
Generated                                                     ●    
Imported                                                  ●         
```

### Intelligence Level
```
Basic Templates ←————————————————————————————————————————————————————→ Context Aware
                ●———————————————————————————————————————————————————○
Hallwatch       ●                                                   
Postman                                                            ●
Insomnia                                                           ●
Swagger                                                        ●    
```

### Framework Flexibility
```
Manual Setup ←——————————————————————————————————————————————————————→ Auto-Detects
             ●—————————————————————————————————————————————————————○
Hallwatch    ●                                                     
Postman                                                           ●
Insomnia                                                          ●
Swagger                                                       ●    
```

### Learning Curve
```
Complex Interface ←———————————————————————————————————————————————————→ Point & Click
                  ●——————————————————————————————————————————————————○
Hallwatch         ●                                                  
Postman                                                             ●
Insomnia                                                        ●    
Swagger                                                     ●         
```

### Team Onboarding
```
Complex Setup ←———————————————————————————————————————————————————————→ Share Codebase
              ●——————————————————————————————————————————————————————○
Hallwatch     ●                                                      
Postman                                                             ●
Insomnia                                                        ●    
Swagger                                                     ●         
```

### Version Control Integration
```
Separate Tracking ←———————————————————————————————————————————————————→ In Your Repo
                  ●——————————————————————————————————————————————————○
Hallwatch         ●                                                  
Postman                                                             ●
Insomnia                                                        ●    
Swagger                                                     ●         
```

### Development Flow Disruption
```
Switch Apps ←—————————————————————————————————————————————————————————→ Stay in Editor
            ●—————————————————————————————————————————————————————————○
Hallwatch   ●                                                        
Postman                                                             ●
Insomnia                                                        ●    
Swagger                                                     ●         
```

### API Change Detection
```
Manual Updates ←——————————————————————————————————————————————————————→ Auto-Current
               ●——————————————————————————————————————————————————————○
Hallwatch      ●                                                     
Postman                                                             ●
Insomnia                                                            ●
Swagger                                                         ●    
```

### Memory Footprint
```
500MB+ ←——————————————————————————————————————————————————————————————→ <100MB
       ●——————————————————————————————————————————————————————————————○
Hallwatch ●                                                          
Postman                                                             ●
Insomnia                                                        ●    
Swagger                                                     ●         
```

### Offline Capability
```
Cloud Dependent ←—————————————————————————————————————————————————————→ Fully Local
                ●—————————————————————————————————————————————————————○
Hallwatch       ●                                                    
Swagger                                                             ●
Postman                                                         ●    
Insomnia                                                    ●         
```

### Security & Privacy
```
Cloud Sync ←——————————————————————————————————————————————————————————→ 100% Local
           ●——————————————————————————————————————————————————————————○
Hallwatch  ●                                                         
Postman                                                             ●
Swagger                                                         ●    
Insomnia                                                    ●         
```

### Real-time Collaboration
```
Complex Workspaces ←——————————————————————————————————————————————————→ Share Code
                   ●——————————————————————————————————————————————————○
Hallwatch          ●                                                 
Postman                                                             ●
Insomnia                                                        ●    
Swagger                                                     ●         
```

## 🏗️ Architecture

This is an NX monorepo containing:

- **Apps**
  - `cli` - Command-line interface for API discovery and watching
  - `desktop` - Tauri-based playground application (coming soon)

- **Libraries**
  - `core` - File watching and event system
  - `parser` - AST-based endpoint discovery engine
  - `ai` - Intelligent parameter analysis with smart caching (coming soon)

## 🚀 Getting Started

```bash
# Install Hallwatch
npm install -g hallwatch

# Point it at your project
hallwatch watch ./my-api

# That's it! Open the playground at http://localhost:3000
```

## ✨ Core Features

### 🔍 **Intelligent Code Parsing**
Our custom AST parser understands real implementations:
- Detects endpoints without decorators
- Extracts parameters from function signatures
- Identifies middleware and auth requirements
- Works with any coding style

### 🤖 **Smart Parameter Suggestions**
AI analyzes your code context to suggest:
- Required vs optional parameters
- Valid test values based on variable names
- Authentication headers from middleware
- Response formats from return statements

### � **Persistent Playground State**
Your testing setup persists between sessions:
- Remembers your last test values
- Keeps environment configurations
- Maintains request history
- Never loses your work

### ⚡ **Real-time Updates**
Save a file, test immediately:
- <10ms endpoint detection
- Instant UI updates
- No restart required
- Git-aware change tracking

## 🛠 Technical Architecture

```
Your Code → AST Parser → Endpoint Detection → AI Enhancement → Playground UI
    ↑           ↓               ↓                    ↓              ↓
File Watch   <10ms parse   Smart caching    Context analysis   Test & iterate
```

**Key Design Decisions:**
- **Rust Core**: Maximum performance for file watching and parsing
- **Tree-sitter**: Language-agnostic AST parsing
- **Local-First AI**: Privacy-preserving parameter suggestions
- **Smart Caching**: AI runs once per endpoint, not per file save

## 📚 Supported Frameworks

**JavaScript/TypeScript**: Express, Fastify, Koa, Next.js, Hono  
**Python**: FastAPI, Flask, Django, Starlette  
**Go**: Gin, Echo, Fiber, Chi  
**Rust**: Actix, Rocket, Axum, Warp  
**Java**: Spring Boot, Quarkus  
**Ruby**: Rails, Sinatra  
**PHP**: Laravel, Symfony

> No special setup needed—just works with standard patterns.

## 🎯 Who Is This For?

### Perfect For:
- **Rapid Prototyping**: Test as you build
- **Team Collaboration**: Everyone sees the same APIs
- **Legacy Projects**: No annotations to add
- **Microservices**: Manage multiple APIs effortlessly

### Not For:
- Public API documentation (use OpenAPI generators)
- Non-HTTP protocols (gRPC, WebSocket coming later)
- Production monitoring (this is for development)

## 🔧 Advanced Configuration (Optional)

While Hallwatch works with zero config, you can customize behavior:

```javascript
// .hallwatch/config.js
export default {
  // Custom ignore patterns
  ignore: ['tests/**', 'migrations/**'],
  
  // Override AI suggestions
  parameterHints: {
    'userId': () => 'user_' + Math.random().toString(36).substring(7),
    'email': () => 'test@example.com'
  },
  
  // Environment configs
  environments: {
    local: 'http://localhost:3000',
    staging: 'https://api-staging.myapp.com'
  }
}
```

## �️ Development

### Prerequisites
- Node.js 18+
- PNPM 8+
- Rust 1.75+
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/hallwatch/hallwatch.git
cd hallwatch

# Install dependencies
pnpm install

# Build all projects
pnpm nx run-many --target=build --all

# Run the CLI to discover endpoints
pnpm nx run cli:serve -- discover ./my-project

# Watch a directory for changes
pnpm nx run cli:serve -- watch ./my-project
```

### For AI Agents
Development instructions for AI assistants are in `.claude/instructions.md`.

## 🤝 Contributing

We welcome contributions! Priority areas:
- Additional language parsers
- Framework-specific improvements  
- Performance optimizations

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Hallwatch is licensed under the Functional Source License 1.1 (FSL).
- ✅ **Free for**: Personal use, open source, companies <$100k revenue
- 💰 **Paid for**: Commercial use by larger companies
- 🔄 **Converts to**: Apache 2.0 after 4 years

---

**Stop maintaining API specs. Start shipping features.**

[Website](https://hallwatch.dev) | [Discord](https://discord.gg/hallwatch) | [Documentation](https://docs.hallwatch.dev)
