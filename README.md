# Hack-n-Roll 2026 Project

## 🎮 Code Arcade - Terminal Language Roulette

A retro arcade-style terminal UI coding challenge game built with Rust and Ratatui!

### Quick Start

```bash
# Run the TUI game
cargo run --release

# Or use the convenience script
./start.sh
```

### What is This?

An interactive terminal application where you solve coding challenges while racing against a 45-second timer that randomly switches your code between programming languages (Python, JavaScript, TypeScript, Rust, Go, Java)!

**Key Features:**
- 🎯 Split-panel interface (problem description + code editor)
- ⏱️ 45-second language randomization timer
- 🎨 Retro arcade aesthetic with animations
- ⌨️ Full text editor with syntax highlighting
- ✅ Submit with Cmd+S for instant test results
- 🌐 6 programming languages supported

### Documentation

- **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - Complete technical overview
- **[CODE_ARCADE_README.md](CODE_ARCADE_README.md)** - Detailed user guide
- **[UI_GUIDE.md](UI_GUIDE.md)** - Visual UI reference and design

### Project Structure

```
├── src/
│   ├── main.rs          # Entry point & terminal setup
│   ├── app.rs           # App state & UI rendering
│   ├── languages.rs     # Language conversion logic
│   └── problem.rs       # Problems & test runner
├── Cargo.toml           # Rust dependencies
├── start.sh             # Quick start script
└── README.md            # This file
```

### Tech Stack

- **Ratatui** - Terminal UI framework
- **Crossterm** - Terminal manipulation
- **Rust** - Systems programming language
- **Tokio** - Async runtime (for future API calls)

### Next.js App (Original)

This workspace also contains a Next.js project in the `app/` directory. To run it:

```bash
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser.

---

**Built for Hack-n-Roll 2026** 🚀

