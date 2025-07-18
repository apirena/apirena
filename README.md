# Apirena: The Code-Aware API Development Environment

Apirena is an intelligent API testing tool that understands your code, not just your specs. It watches your source files, automatically discovers endpoints through AST parsing, and uses AI to help you build better APIs—all in real-time as you code.

## Why Apirena?

**The Problem**: Current API tools are disconnected from your code. You manually maintain collections, copy-paste URLs, and hope your documentation matches reality.

**Our Solution**: Apirena watches your actual source files and understands your code structure. Change a route? Apirena knows instantly. Add a parameter? It's already in the test builder. Write a comment? That's your documentation.

![MacBook Pro 14_ - 1](https://github.com/apirena/apirena/assets/23046374/736622da-fdad-45b6-b18d-6c6f794318e9)

# Apirena: The Code-Aware API Development Environment

Apirena is an intelligent API testing tool that understands your code, not just your specs. It watches your source files, automatically discovers endpoints through AST parsing, and uses AI to help you build better APIs—all in real-time as you code.

## Why Apirena?

**The Problem**: Current API tools are disconnected from your code. You manually maintain collections, copy-paste URLs, and hope your documentation matches reality.

**Our Solution**: Apirena watches your actual source files and understands your code structure. Change a route? Apirena knows instantly. Add a parameter? It's already in the test builder. Write a comment? That's your documentation.

## ✨ Key Features

### 🧠 **Automatic Endpoint Discovery**
```javascript
// You write:
app.post('/users/:id/avatar', uploadAvatar);

// Apirena instantly shows:
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
$ apirena diff main..feature/auth
> Added: POST /auth/refresh
> Modified: POST /login (added mfa_token field)
```

### 🚀 **Zero-Latency Updates**
File changes reflect instantly. No refresh. No rebuild. Just save and test.

## 🎯 Getting Started

```bash
# Download for your platform
curl -L https://github.com/apirena/apirena/releases/latest/download/apirena-{os} -o apirena
chmod +x apirena

# Run in your project
./apirena

# That's it. Apirena finds your APIs automatically.
```

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

Don't see yours? [Open an issue](https://github.com/apirena/apirena/issues)—adding frameworks is easy with Tree-sitter.

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

## 🏗 Architecture

- **Core**: Rust for performance and reliability
- **Parser**: Tree-sitter for language understanding  
- **UI**: Svelte 5 with reactive state
- **Desktop**: Tauri 2 for native feel
- **AI**: Local LLM support (Ollama) with cloud fallback

## 🤝 Contributing

We welcome contributions! Priority areas:
- Additional language parsers
- Framework detection patterns
- UI/UX improvements

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Apirena is licensed under the Business Source License 1.1 (BSL).

- ✅ **Free for**: Open source projects, personal use, testing, and evaluation
- ✅ **Free for**: Companies with <$100k annual revenue  
- 💰 **Paid license required for**: Commercial use by companies >$100k revenue
- 🔄 **Converts to**: Apache 2.0 after 4 years

See [LICENSE](LICENSE) for full terms.

## 🙏 Acknowledgments

Built with amazing open source projects:
- [Tree-sitter](https://tree-sitter.github.io/) for parsing
- [Tauri](https://tauri.app/) for desktop apps
- [Svelte](https://svelte.dev/) for reactive UI

---

**Stop syncing collections. Start shipping APIs.**

[Website](https://apirena.dev) | [Discord](https://discord.gg/apirena) | [Documentation](https://docs.apirena.dev)
