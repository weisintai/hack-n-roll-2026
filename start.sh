#!/bin/bash
# Quick start script for Code Arcade

echo "🎮⚡ Starting Code Arcade - Language Roulette ⚡🎮"
echo ""
echo "Controls:"
echo "  - Type to edit code"
echo "  - Arrow keys to navigate"
echo "  - Cmd+S / Ctrl+S to submit"
echo "  - Timer: 45 seconds until language randomize!"
echo ""
echo "Press any key to start..."
read -n 1

cargo run --release
