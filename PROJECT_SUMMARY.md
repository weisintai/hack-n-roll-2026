# Code Arcade - Terminal Language Roulette 🎮⚡

## Quick Start

```bash
# Run the game
cargo run --release

# Or use the start script
./start.sh
```

## What You've Got

A fully functional **terminal-based coding arcade game** built with Ratatui (Rust TUI framework) that features:

### ✨ Core Features

1. **Split-Panel Interface** (33/67 layout)
   - **Left panel**: LeetCode "Two Sum" problem with description, examples, and constraints
   - **Right panel**: Live code editor with line numbers and syntax coloring

2. **Language Roulette Timer** ⏱️
   - 45-second countdown before automatic language randomization
   - Color-coded timer (green → yellow → red as time runs out)
   - Animated glitch transition effect during conversion

3. **Multi-Language Support** 🌐
   - Python, JavaScript, TypeScript, Rust, Go, Java
   - Mock code conversion (ready to plug in LLM API)
   - Each language maintains proper syntax in the editor

4. **Full-Featured Text Editor** ⌨️
   - Type, backspace, tab support
   - Arrow key navigation (up/down/left/right)
   - Multi-line editing
   - Cursor position tracking

5. **Submission & Testing** ✅
   - Press `Cmd+S` (Mac) or `Ctrl+S` (Windows/Linux) to submit
   - Auto-converts code to Python for execution
   - Runs against 4 test cases
   - Shows detailed pass/fail results

6. **Retro Arcade Aesthetic** 🎨
   - ASCII art borders and headers
   - Colorful terminal UI (Cyan, Magenta, Yellow, Green)
   - Glitch effects during transitions
   - Score screen with percentage calculation

### 🏗️ Architecture

```
src/
├── main.rs          # Entry point, terminal setup, main event loop
├── app.rs           # App state, UI rendering, input handling
├── languages.rs     # Language enum and mock code converters
└── problem.rs       # Problem definitions and test runner
```

**Key Components:**

- **App State Machine**: `Coding` → `Transitioning` → `Coding` (or `Results`)
- **Event Loop**: 100ms polling for smooth animations
- **Mock APIs**: Ready to swap with real LLM for code conversion

### 🎯 How It Works

1. **Coding Phase**
   - User writes code in current language
   - Timer counts down from 45 seconds
   - All keyboard input captured (typing, navigation)

2. **Transition Phase** (2 seconds)
   - Animated glitch effect fills screen
   - Progress bar shows conversion status
   - Code converts to random different language
   - No input accepted during transition

3. **Results Phase**
   - Triggered by `Cmd+S` submission
   - Shows test results with score
   - Options to restart (`R`) or quit (`Q`)

### 🔧 Customization Points

**Change Timer Duration:**
```rust
// In src/app.rs, line ~39
randomize_interval: Duration::from_secs(45),  // Change to your preference
```

**Add More Problems:**
```rust
// In src/problem.rs
impl Problem {
    pub fn your_problem() -> Self {
        // Define new problem, examples, test cases
    }
}
```

**Hook Up Real LLM:**
```rust
// In src/languages.rs, convert_code() function
// Replace mock converters with API calls to GPT/Claude/etc.
pub async fn convert_code(code: &str, from: Language, to: Language) -> String {
    // Your LLM API call here
}
```

### 📦 Dependencies

- `ratatui 0.27` - Terminal UI framework
- `crossterm 0.27` - Cross-platform terminal control
- `tokio` - Async runtime (for future API calls)
- `serde/serde_json` - Serialization
- `rand` - Random language selection
- `anyhow` - Error handling

### 🎮 Controls Reference

| Key | Action |
|-----|--------|
| Type normally | Insert characters |
| `Enter` | New line |
| `Tab` | Insert tab |
| `Backspace` | Delete character |
| `Arrow Keys` | Move cursor |
| `Cmd+S` / `Ctrl+S` | Submit solution |
| `R` (results) | Restart game |
| `Q` (results) | Quit |
| `Esc` (results) | Quit |

### 🚀 Next Steps

**Easy Additions:**
- [ ] Add more LeetCode problems
- [ ] Implement difficulty selection
- [ ] Add sound effects (terminal beeps)
- [ ] High score tracking
- [ ] More languages (C++, C#, Ruby, etc.)

**Medium Complexity:**
- [ ] Real LLM integration (OpenAI, Anthropic, etc.)
- [ ] Actual code execution in Docker containers
- [ ] Persistent settings/preferences
- [ ] Custom problem input

**Advanced:**
- [ ] Multiplayer mode (race against others)
- [ ] Leaderboard API
- [ ] Problem difficulty scaling
- [ ] Code quality metrics

### 🐛 Troubleshooting

**Terminal looks weird after crash:**
```bash
reset
```

**Can't see anything:**
- Make sure your terminal supports colors (most modern terminals do)
- Try resizing your terminal window

**Build errors:**
- Ensure Rust 1.87+ is installed: `rustc --version`
- Clean and rebuild: `cargo clean && cargo build --release`

### 📝 Technical Notes

- Uses alternate screen buffer (terminal state restored on exit)
- Raw mode for character-by-character input
- 100ms tick rate for smooth animations
- Supports Unicode text editing
- Cross-platform (macOS, Linux, Windows)

### 🎉 What Makes This Special

This isn't just a code editor - it's an **arcade experience**:
- **Time pressure**: 45-second countdown creates urgency
- **Randomization**: Forces you to think across languages
- **Visual feedback**: Retro aesthetics make it fun
- **Instant results**: See your score immediately
- **Fully terminal-based**: No GUI, pure hacker vibes

Perfect for:
- Coding practice with added challenge
- Learning language syntax patterns
- Demo/presentation of TUI capabilities
- Hackathon projects
- Portfolio piece

---

**Built with ❤️ using Rust and Ratatui**

*Ready to play? Run `cargo run --release` and start coding! 🚀*
