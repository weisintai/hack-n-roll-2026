#!/bin/bash
# Demo script for Code Arcade

clear

echo "════════════════════════════════════════════════════════════════"
echo "           🎮 CODE ARCADE - TERMINAL LANGUAGE ROULETTE 🎮        "
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "This is a fully functional TUI-based coding challenge game!"
echo ""
echo "✨ Features:"
echo "   • Split-panel interface (problem | code editor)"
echo "   • 15-second timer with language randomization"
echo "   • Supports Python, JS, TS, Rust, Go, Java"
echo "   • Full text editing with arrow keys"
echo "   • Submit with Cmd+S for instant results"
echo "   • Retro arcade aesthetic with animations"
echo ""
echo "📝 How to Play:"
echo "   1. Read the LeetCode problem (Two Sum) on the left"
echo "   2. Write your solution in the editor on the right"
echo "   3. Every 45 seconds, your code randomly converts!"
echo "   4. Press Cmd+S (or Ctrl+S) when you're ready"
echo "   5. See your test results and score"
echo ""
echo "⌨️  Quick Controls:"
echo "   • Type normally to edit"
echo "   • Arrow keys to navigate"
echo "   • Cmd+S / Ctrl+S to submit"
echo "   • R to restart (after results)"
echo "   • Q to quit (after results)"
echo ""
echo "🔧 Technical Stack:"
echo "   • Ratatui (Rust TUI framework)"
echo "   • Crossterm (terminal control)"
echo "   • Built with ❤️  in Rust"
echo ""
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Building the application (this may take a moment)..."
echo ""

cd "$(dirname "$0")"

if cargo build --release 2>&1 | grep -q "Finished"; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "Ready to launch in 3 seconds..."
    sleep 1
    echo "2..."
    sleep 1
    echo "1..."
    sleep 1
    echo ""
    echo "🚀 LAUNCHING CODE ARCADE! 🚀"
    echo ""
    sleep 1
    
    ./target/release/code_arcade
    
    echo ""
    echo "════════════════════════════════════════════════════════════════"
    echo "Thanks for playing Code Arcade! 🎮"
    echo "════════════════════════════════════════════════════════════════"
else
    echo ""
    echo "❌ Build failed. Please check the error messages above."
    echo ""
    echo "Try running: cargo clean && cargo build --release"
    exit 1
fi
