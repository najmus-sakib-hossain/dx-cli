# 🤖 Dx AI Chat Interface

## Overview

When you run `dx` without any arguments, you'll enter an **interactive AI chat interface** powered by Google Gemini. This provides a unified experience for:
- **Chatting with AI** - Ask questions, get coding help, explanations
- **Running shell commands** - Execute commands directly from the chat
- **Running dx commands** - Access all dx tools from the chat

---

## Quick Start

```bash
# Just type dx to start
dx

# You'll see the interactive chat interface:
┌─ Dx AI Chat ────────────────────────────────────────┐
│ • System                                             │
│ Dx AI Assistant - Type commands or ask questions.   │
│ Press Ctrl+C to exit.                                │
│                                                      │
└──────────────────────────────────────────────────────┘
┏━ Type your message or command... (Ctrl+C to exit) ━━┓
┃ █                                                    ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

## Features

### 🤖 AI-Powered Chat
- **Google Gemini Integration** - Free `gemini-1.5-flash` model
- **Conversation History** - Maintains context across messages
- **Streaming Responses** - Real-time AI responses with loading indicator
- **Smart Context** - AI understands previous messages

### 💻 Command Execution
- **Shell Commands** - Run any shell command directly
  ```bash
  ls -la
  cargo build
  git status
  ```

- **Dx Commands** - Access all dx tools
  ```bash
  dx ui add button
  dx shell --enable
  dx config get-api-key
  ```

### ⌨️ Keyboard Controls
- **Enter** - Send message or execute command
- **Ctrl+C** - Exit chat
- **Up/Down** - Scroll through message history
- **Backspace** - Delete characters

---

## Command Detection

The chat automatically detects and executes:

### Shell Commands
```
ls, cd, pwd, cat, echo, mkdir, rm, cp, mv, grep
cargo, npm, yarn, git, python, node, rustc, code
```

### Dx Commands
```
dx ui, dx style, dx icon, dx font, dx auth
dx media, dx i18n, dx forge, dx generate, dx shell
```

**Any other input** is sent to the AI for a response.

---

## API Key Configuration

### Using Default Key
By default, dx uses a built-in Gemini API key. This works out of the box!

### Setting Your Own Key

```bash
# Set your Gemini API key
dx config set-api-key YOUR_GEMINI_API_KEY_HERE

# Verify it's set
dx config get-api-key
```

### Getting a Free API Key

1. Go to [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Click "Create API Key"
3. Copy your key
4. Run `dx config set-api-key <YOUR_KEY>`

The free tier includes:
- 15 requests per minute
- 1,500 requests per day
- No credit card required

---

## Usage Examples

### Example 1: Chat with AI

```
▶ You
How do I create a React component with TypeScript?

● Assistant
Here's how to create a React component with TypeScript:

1. Create a new file: `MyComponent.tsx`
2. Define your component:

```tsx
import React from 'react';

interface MyComponentProps {
  title: string;
  onClick?: () => void;
}

export const MyComponent: React.FC<MyComponentProps> = ({ 
  title, 
  onClick 
}) => {
  return (
    <div onClick={onClick}>
      <h1>{title}</h1>
    </div>
  );
};
```

This creates a typed functional component with props interface.
```

### Example 2: Execute Shell Command

```
▶ You
ls -la

$ Command
Executing: ls -la

$ Command
total 32
drwxr-xr-x  8 user  staff   256 Dec  9 10:30 .
drwxr-xr-x  5 user  staff   160 Dec  8 15:20 ..
-rw-r--r--  1 user  staff  1234 Dec  9 10:25 Cargo.toml
drwxr-xr-x  12 user  staff   384 Dec  9 10:30 crates
-rw-r--r--  1 user  staff  5678 Dec  9 09:15 README.md
```

### Example 3: Mixed Workflow

```
▶ You
What files are in this directory?

● Assistant
I can help you see the files. Would you like me to list them? 
You can type `ls` to see all files, or `ls -la` for detailed info.

▶ You
ls

$ Command
Executing: ls

$ Command
Cargo.toml  crates/  README.md  target/  docs/

▶ You
What's in the crates directory?

● Assistant
The crates directory typically contains Rust workspace crates.
You can explore it with `ls crates/` to see the individual crates.

▶ You
ls crates/

$ Command
Executing: ls crates/

$ Command
dx-ai/  dx-cli/  dx-core/  dx-tui/  dx-shell/  dx-generator/
```

---

## Message Types

The chat uses different colors for different message types:

| Type | Color | Example |
|------|-------|---------|
| **▶ You** | Blue (Vercel) | Your messages |
| **● Assistant** | Green | AI responses |
| **• System** | Gray | System messages |
| **$ Command** | Yellow | Command execution |

---

## Tips & Tricks

### 1. Quick Command Execution
Type commands directly without any prefix:
```
git status
cargo test
npm run dev
```

### 2. Ask for Explanations
```
What does this error mean?
How do I fix CORS issues?
Explain Rust ownership
```

### 3. Code Generation
```
Generate a Python function to sort a list
Create a React hook for API calls
Write a bash script to backup files
```

### 4. Documentation Lookup
```
How do I use tokio::spawn?
What's the difference between Arc and Rc?
Explain CSS flexbox
```

### 5. Debugging Help
```
Why is my cargo build failing?
How do I fix this TypeScript error?
What's wrong with this SQL query?
```

---

## Advanced Usage

### Custom AI Model

Edit `~/.dx/config.toml`:
```toml
[ai]
model = "gemini-1.5-pro"  # Use pro model for better responses
temperature = 0.9          # More creative (0.0 - 1.0)
max_tokens = 4096          # Longer responses
```

### History Persistence

Message history is kept **in-memory only** during the session. When you exit (Ctrl+C), history is cleared.

Future enhancement: Persistent conversation history across sessions.

---

## Troubleshooting

### Chat Won't Launch
```bash
# Make sure dx-cli is built
cargo build --release

# Run directly
./target/release/dx
```

### API Key Errors
```bash
# Check if key is set
dx config get-api-key

# Set new key
dx config set-api-key YOUR_KEY

# Test with a message
dx
# Type: "Hello" and press Enter
```

### Commands Not Executing
- Make sure the command is in the detected command list
- Check that the command exists on your system
- Try using absolute paths for executables

### AI Not Responding
- Check your internet connection
- Verify API key is valid
- Check Gemini API rate limits (15/min, 1500/day)

---

## Comparison with Other CLIs

| Feature | Dx AI Chat | Claude CLI | Gemini CLI |
|---------|------------|------------|------------|
| **AI Provider** | Google Gemini | Anthropic Claude | Google Gemini |
| **Command Execution** | ✅ Built-in | ❌ External | ❌ External |
| **Free Tier** | ✅ Yes | ❌ Limited | ✅ Yes |
| **Offline Commands** | ✅ Yes | ❌ No | ❌ No |
| **Shell Integration** | ✅ Yes | ❌ No | ❌ No |
| **Tool Access** | ✅ All dx tools | ❌ No | ❌ No |

---

## Keyboard Reference

| Key | Action |
|-----|--------|
| `Enter` | Send message or execute command |
| `Ctrl+C` | Exit chat interface |
| `↑` | Scroll up in message history |
| `↓` | Scroll down in message history |
| `Backspace` | Delete character |
| `Any key` | Type character |

---

## Configuration File

Location: `~/.dx/config.toml`

```toml
[ai]
gemini_api_key = "YOUR_API_KEY_HERE"
model = "gemini-1.5-flash"
temperature = 0.7
max_tokens = 2048
```

---

## Future Enhancements

- [ ] Persistent conversation history
- [ ] Multi-session management
- [ ] Code syntax highlighting in responses
- [ ] File attachment support
- [ ] Voice input/output
- [ ] Custom AI personas
- [ ] Streaming token-by-token responses
- [ ] Conversation export (markdown, JSON)

---

## Examples Gallery

### Code Review
```
▶ You
Review this Rust code: [paste code]

● Assistant
Here's my review:

1. ✅ Good use of Result type
2. ⚠️  Consider using &str instead of String
3. ❌ Missing error handling on line 15
4. 💡 Suggestion: Use iterator chains

[detailed review continues...]
```

### Learning Assistant
```
▶ You
Explain async/await in Rust

● Assistant
Async/await in Rust allows you to write asynchronous code
that looks synchronous. Here's how it works:

1. `async fn` - Marks a function as asynchronous
2. `.await` - Waits for an async operation to complete
3. Runtime (tokio/async-std) - Executes async tasks

Example:
```rust
async fn fetch_data() -> Result<String> {
    let response = reqwest::get("https://api.example.com")
        .await?;
    response.text().await
}
```
```

### Project Setup
```
▶ You
Set up a new Rust project

$ Command
Executing: cargo new my-project

$ Command
Created binary (application) `my-project` package

● Assistant
Great! Your Rust project is ready. Next steps:

1. cd my-project
2. cargo run
3. Edit src/main.rs to add your code

Would you like me to explain the project structure?
```

---

**For more information, see:**
- [QUICKSTART.md](QUICKSTART.md) - Getting started guide
- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Implementation status
- [README.md](README.md) - Main project README
