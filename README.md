# Hallwatch: The Code-Aware API Development Environment

Hallwatch is an intelligent API testing tool that understands your code, not just your specs. It watches your source files, automatically discovers endpoints through AST parsing, and uses AI to help you build better APIs—all in real-time as you code.

## 🏗️ Architecture

This is an NX monorepo containing:

- **Apps**
  - `cli` - Command-line interface for API discovery and watching
  - `desktop` - Tauri-based desktop application (coming soon)

- **Libraries**
  - `core` - File watching and event system
  - `parser` - Tree-sitter based code parsing for endpoint discovery
  - `git` - Git integration for API diffing (coming soon)
  - `ai` - AI-powered test generation (coming soon)

## 🚀 Getting Started

### Prerequisites
- Node.js 18+
- PNPM 8+
- Rust 1.70+
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

## 🛠️ Development

### Running Tasks

```bash
# Run unit tests for all libraries
pnpm nx run-many --target=test --all

# Run integration tests for parser
pnpm nx integration-test parser

# Run integration tests for core
pnpm nx integration-test core

# Run granular tests for specific frameworks
pnpm nx run cli:test:php:laravel
pnpm nx run cli:test:javascript:express
pnpm nx run cli:test:python:fastapi

# Run tests for affected projects only
pnpm nx affected:test

# Build a specific library
pnpm nx build parser

# See dependency graph
pnpm nx graph
```

### Adding New Features

```bash
# Generate a new Rust library
pnpm nx g @monodon/rust:library my-feature --directory=libs

# Run clippy on all Rust code
pnpm nx run-many --target=clippy --all
```

## Why Hallwatch?

**The Problem**: Current API tools are disconnected from your code. You manually maintain collections, copy-paste URLs, and hope your documentation matches reality.

**Our Solution**: Hallwatch watches your actual source files and understands your code structure. Change a route? Hallwatch knows instantly. Add a parameter? It's already in the test builder. Write a comment? That's your documentation.

## ✨ Key Features

### 🧠 **Automatic Endpoint Discovery**
```javascript
// You write:
app.post('/users/:id/avatar', uploadAvatar);

// Hallwatch instantly shows:
// POST /users/:id/avatar [Test]
```
No configuration. No manual updates. It just works.

### 📝 **Natural Language Documentation**
```python
# This endpoint sends a welcome email
# Requires: user_id, template_name  
# Returns: email_id or error
@app.route('/emails/send', methods=['POST'])
```
Write comments naturally. AI understands your intent—no strict syntax required.

### 🤖 **Intelligent Test Assistance**
- Suggests test values based on your code patterns
- Remembers recent tests for quick replay
- Generates edge cases from parameter types

### 🔄 **Git-Aware Development**
```bash
# See what changed between branches
$ hallwatch diff main..feature/auth
> Added: POST /auth/refresh
> Modified: POST /login (added mfa_token field)
```

### 🌐 **Public API Superpowers**

Monitor any public API for changes:

```javascript
// .hallwatch/watch.json
{
  "public_apis": [
    {
      "name": "Stripe",
      "source": "https://stripe.com/docs/api",
      "notify": ["breaking_changes", "new_endpoints"],
      "sync_schedule": "daily"
    },
    {
      "name": "OpenAI", 
      "source": "https://platform.openai.com/docs",
      "track_pricing": true
    }
  ]
}
```

Get notified when:
- Stripe adds new endpoints
- OpenAI changes their rate limits  
- Any dependency updates their API

The AI continuously learns from public API patterns to improve suggestions for your private APIs.

### 🚀 **Zero-Latency Updates**
File changes reflect instantly. No refresh. No rebuild. Just save and test.

## 🛠 How It Works

1. **Watch** - Monitors your source files using native file system events
2. **Parse** - Tree-sitter extracts API patterns in <10ms
3. **Understand** - AI interprets comments and suggests improvements
4. **Test** - Interactive playground updates as you type

No servers. No syncing. Everything runs locally on your machine.

## 📚 Supported Frameworks

**JavaScript/TypeScript**: Express, Fastify, Koa, Next.js, Hono  
**Python**: FastAPI, Flask, Django  
**Go**: Gin, Echo, Fiber  
**Rust**: Actix, Rocket, Axum  
**Ruby**: Rails, Sinatra  
**Java**: Spring Boot  
**PHP**: Laravel, Symfony

Don't see yours? [Open an issue](https://github.com/hallwatch/hallwatch/issues)—adding frameworks is easy with Tree-sitter.

## 💡 Perfect For

### During Development
- Test endpoints without leaving your flow
- Catch breaking changes before commits
- Document as you code with natural comments

### Code Reviews  
- See exact API changes in PRs
- Share test sessions with teammates
- Validate API design decisions

### AI-Assisted Coding
- Validates Copilot/Cursor generated endpoints
- Tests AI-written APIs automatically  
- Human-in-the-loop verification

## 🏗 Technical Architecture

- **Core**: Rust for performance and reliability
- **Parser**: Tree-sitter for language understanding  
- **UI**: Svelte 5 with reactive state
- **Desktop**: Tauri 2 for native feel
- **AI**: Local LLM support (Ollama) with cloud fallback
- **Monorepo**: NX for efficient development and builds

## 📚 Documentation

- [Architecture Overview](./docs/ARCHITECTURE.md)
- [Development Guide](./docs/DEVELOPMENT.md)
- [AI Agent Guidelines](./docs/AI_GUIDELINES.md)
- [Development Milestones](./docs/MILESTONES.md)

## 🤝 Contributing

We welcome contributions! Priority areas:
- Additional language parsers
- Framework detection patterns
- UI/UX improvements

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Hallwatch is licensed under the Functional Source License 1.1 (FSL).

- ✅ **Free for**: Open source projects, personal use, testing, and evaluation
- 💰 **Paid license required for**: Commercial use by companies >$100k revenue
- 🔄 **Converts to**: Apache 2.0 after 4 years

See [LICENSE](LICENSE) for full terms.

## 🙏 Acknowledgments

Built with amazing open source projects:
- [Tree-sitter](https://tree-sitter.github.io/) for parsing
- [Tauri](https://tauri.app/) for desktop apps
- [Svelte](https://svelte.dev/) for reactive UI
- [NX](https://nx.dev/) for monorepo management

---

**Stop syncing collections. Start shipping APIs.**

[Website](https://hallwatch.dev) | [Discord](https://discord.gg/hallwatch) | [Documentation](https://docs.hallwatch.dev)
