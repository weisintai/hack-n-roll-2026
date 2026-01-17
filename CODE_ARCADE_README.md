# Code Arcade - Terminal Language Roulette 🎮⚡

A retro arcade-style terminal UI coding challenge game that randomly switches your code between programming languages!

## Features

🎯 **Split-View Interface**
- Left panel (1/3): LeetCode-style problem description
- Right panel (2/3): Code editor with syntax highlighting

⏱️ **45-Second Language Roulette**
- Every 45 seconds, your code gets randomly converted to a different language
- Animated glitch transition effect while "converting"
- Supports: Python, JavaScript, TypeScript, Rust, Go, Java

⌨️ **Interactive Editor**
- Full text editing capabilities
- Arrow key navigation
- Line numbers
- Press `Cmd+S` (or `Ctrl+S`) to submit your solution

🧪 **Test Execution**
- Automatically converts your code to Python for testing
- Runs against predefined test cases
- Shows detailed results with pass/fail for each test
- Arcade-style results screen with score

🎨 **Retro Terminal Aesthetic**
- Colorful ASCII art borders
- Glitch effects during transitions
- Timer countdown with color coding
- Arcade-inspired UI elements

## Installation

Make sure you have Rust installed. Then:

\`\`\`bash
# Build the project
cargo build --release

# Run the game
cargo run --release
\`\`\`

## How to Play

1. **Launch the game**: Run `cargo run --release`

2. **Read the problem**: The left panel shows a coding challenge (Two Sum problem)

3. **Write your solution**: Type your code in the right panel
   - Use arrow keys to navigate
   - Code starts in Python

4. **Beat the timer**: You have 45 seconds before your code randomly switches languages!
   - Timer shown at the bottom of the screen
   - Code automatically converts (with cool glitch effect)

5. **Submit when ready**: Press `Cmd+S` (Mac) or `Ctrl+S` (Linux/Windows)
   - Your code converts to Python
   - Runs against test cases
   - See your score!

6. **View results**: After submission
   - See which tests passed/failed
   - View your score percentage
   - Press `R` to restart or `Q` to quit

## Controls

- **Arrow Keys**: Navigate cursor
- **Type normally**: Edit code
- **Enter**: New line
- **Tab**: Insert tab
- **Backspace**: Delete character
- **Cmd+S / Ctrl+S**: Submit solution
- **R** (results screen): Restart game
- **Q** (results screen): Quit game
- **Esc** (results screen): Quit game

## Architecture

The application is built with:
- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **syntect**: Syntax highlighting (future enhancement)
- **tokio**: Async runtime for API calls
- **serde**: JSON serialization

### Project Structure

\`\`\`
src/
├── main.rs          # Entry point, terminal setup, main loop
├── app.rs           # App state, rendering, input handling
├── languages.rs     # Language conversion logic (mock API)
└── problem.rs       # Problem definitions, test runner
\`\`\`

## Customization

### Change the Timer Interval

In `src/app.rs`, modify:
\`\`\`rust
randomize_interval: Duration::from_secs(45),  // Change 45 to your desired seconds
\`\`\`

### Add More Languages

In `src/languages.rs`:
1. Add your language to the `Language` enum
2. Implement conversion functions
3. Update the mock converters

### Add More Problems

In `src/problem.rs`:
1. Create a new problem method like `Problem::two_sum()`
2. Define test cases
3. Update `App::new()` to use your problem

## Future Enhancements

- [ ] Real LLM API integration for code conversion
- [ ] Actual code execution in sandboxed environments
- [ ] More LeetCode problems
- [ ] Difficulty levels
- [ ] Leaderboard system
- [ ] Sound effects (terminal beeps)
- [ ] More languages
- [ ] Custom problem input

## Development

\`\`\`bash
# Run in debug mode (faster compilation)
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Run linter
cargo clippy
\`\`\`

## Troubleshooting

**Terminal looks weird after crash**: Run `reset` in your terminal

**Can't see cursor**: The app hides the cursor - press Ctrl+C to exit and it will restore

**Timer not updating**: Make sure your terminal supports color and animations

## License

MIT License - Feel free to modify and extend!

## Credits

Built with ❤️ using Ratatui and the Rust terminal ecosystem.

---

**Have fun coding in the arcade! 🎮⚡**
