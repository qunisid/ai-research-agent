# 🔍 AI Research Agent v0.1.0

A production-ready AI research agent built with **Rust** and the **Rig framework**. 

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![AI](https://img.shields.io/badge/AI-Ollama-green.svg)

Repository: https://github.com/qunisid/ai-research-agent

## ✨ Features

- 🤖 **Local LLM Support** - Uses Ollama for privacy-friendly, free AI inference
- 🔎 **Web Search** - DuckDuckGo integration (no API key required!)
- 🛠️ **Tool-Using Agent** - Demonstrates agentic AI patterns
- 💬 **Interactive Mode** - REPL-style continuous conversation with memory
- 📚 **Beginner Friendly** - Extensive comments explaining Rust patterns
- 🚀 **Production Ready** - Proper error handling, logging, and CLI

## 🚀 Quick Start

### Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Ollama**:
   - Visit [ollama.ai](https://ollama.ai) and follow installation instructions
   - Or on Linux: `curl -fsSL https://ollama.com/install.sh | sh`

3. **Pull a model**:
   ```bash
   ollama pull llama3.2
   # Or any other model you prefer:
   # ollama pull deepseek-v3.2
   # ollama pull qwen3-coder
   ```

4. **Start Ollama**:
   ```bash
   ollama serve
   ```

### Installation

```bash
# Clone the repository
git clone https://github.com/qunisid/ai-research-agent.git
cd ai-research-agent

# Copy environment template
cp .env.example .env

# Build the project
cargo build --release
```

### Usage

**Single Query Mode:**
```bash
# Basic research query
cargo run -- "What are the latest developments in Rust async runtime?"

# Quick search mode (no AI synthesis)
cargo run --release -- --quick "Rust web frameworks 2024"

# Use a specific model
cargo run -- --model deepseek-v3.2 "Machine learning in Rust"

# Verbose output
cargo run -- --verbose "WebAssembly trends"
```

**Interactive Mode (REPL):**
```bash
cargo run -- --interactive
# or
cargo run -- -i
```

In interactive mode:
- Type your question and press Enter
- Ask follow-up questions (AI remembers context)
- Type `clear` to clear conversation history
- Type `quit` or `exit` to quit

```bash
$ cargo run -- -i

============================================================
AI Research Agent - Interactive Mode
============================================================
Type your question and press Enter.
Commands: 'clear' to clear history, 'quit' or 'exit' to quit.
============================================================

You: What is Rust?
[AI responds with search results...]

You: Why is it popular?
[AI remembers previous context and answers...]

You: quit
Goodbye!
```

**Show help:**
```bash
cargo run -- --help
```

## 📁 Project Structure

```
ai-research-agent/
├── Cargo.toml          # Project dependencies and metadata
├── .env.example        # Environment variable template
├── README.md           # This file
└── src/
    ├── main.rs         # CLI entry point and application logic
    ├── config.rs       # Configuration management
    ├── agent.rs        # Research agent implementation
    └── tools.rs        # Web search tool (DuckDuckGo)
```

## 🔧 Configuration

Edit `.env` to customize the agent:

```bash
# Model to use (must be installed in Ollama)
OLLAMA_MODEL=llama3.2

# Ollama server URL
OLLAMA_HOST=http://localhost:11434

# Response creativity (0.0 = focused, 1.0 = creative)
TEMPERATURE=0.7

# Number of search results to analyze
MAX_SEARCH_RESULTS=5

# Logging level
RUST_LOG=info
```

## 🎓 Learning Rust Concepts

This codebase demonstrates these Rust concepts with inline comments:

| Concept | File | Description |
|---------|------|-------------|
| **Structs & Enums** | `config.rs` | Data types and pattern matching |
| **Traits** | `tools.rs` | Implementing the Rig `Tool` trait |
| **Ownership & Borrowing** | `agent.rs` | Memory safety without GC |
| **Async/Await** | `agent.rs`, `tools.rs` | Non-blocking I/O |
| **Error Handling** | All files | `Result`, `?` operator, `anyhow` |
| **Derive Macros** | All files | `Debug`, `Clone`, `Serialize` |
| **Unit Tests** | All files | The `#[cfg(test)]` pattern |

## 🛠️ Extending the Agent

### Adding a New Tool

1. Create a new struct in `tools.rs`:
   ```rust
   pub struct MyNewTool {
       // fields
   }
   ```

2. Implement the `Tool` trait:
   ```rust
   impl Tool for MyNewTool {
       const NAME: &'static str = "my_tool";
       // ... implement required methods
   }
   ```

3. Register with the agent in `agent.rs`:
   ```rust
   let agent = client
       .agent(&model)
       .tool(web_search_tool)
       .tool(my_new_tool)  // Add here
       .build();
   ```

### Using Different Models

Any Ollama-compatible model works:
```bash
ollama pull mistral
ollama pull codellama
ollama pull gemma2
```

Then set `OLLAMA_MODEL` in `.env` or use `--model` flag.

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config
```

## 📊 Sample Output

```
$ cargo run -- "What is WebAssembly?"

============================================================
RESEARCH RESULTS
============================================================

## Overview
WebAssembly (Wasm) is a binary instruction format designed for...

## Key Findings
1. **Performance**: Near-native execution speed...
2. **Portability**: Runs on any platform with a Wasm runtime...
3. **Security**: Sandboxed execution environment...

## Sources
- https://webassembly.org/
- https://developer.mozilla.org/en-US/docs/WebAssembly
- ...

============================================================
```

## 🐛 Troubleshooting

### "Connection refused" error
Make sure Ollama is running:
```bash
ollama serve
```

### "Model not found" error
Pull the model first:
```bash
ollama pull llama3.2
```

### Slow responses
- Try a smaller model: `ollama pull gemma2:2b`
- Check your hardware - LLMs need significant RAM/VRAM

## 📜 License

MIT License - feel free to use this for learning and building!

## 🙏 Acknowledgments

- [Rig Framework](https://rig.rs) - The Rust AI framework
- [Ollama](https://ollama.ai) - Local LLM runner
- [DuckDuckGo](https://duckduckgo.com) - Privacy-respecting search
