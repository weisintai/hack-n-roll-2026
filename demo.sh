#!/bin/bash
# Terminal of Babel - Demo Script

clear

echo ""
echo "  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "  ┃               TERMINAL  of  BABEL                         ┃"
echo "  ┃           Code in any language. Survive.                  ┃"
echo "  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo ""
echo "  A TUI coding challenge where your language changes mid-solve."
echo ""
echo "  Features"
echo "  ────────────────────────────────────────"
echo "    Split-panel interface (challenge | editor)"
echo "    15-second timer with random language shifts"
echo "    12 languages: Python, JS, TS, Rust, Go, Java..."
echo "    Real-time compilation via Piston API"
echo ""
echo "  How to Play"
echo "  ────────────────────────────────────────"
echo "    1. Read the problem on the left"
echo "    2. Write your solution on the right"
echo "    3. Every 45s, your code converts to a new language"
echo "    4. Press ^S to submit when ready"
echo ""
echo "  Controls"
echo "  ────────────────────────────────────────"
echo "    ^S  Submit      ^R  New problem"
echo "    ^C  Compile     ^Q  Quit"
echo ""
echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  Building..."
echo ""

cd "$(dirname "$0")"

if cargo build --release 2>&1 | grep -q "Finished"; then
    echo "  Build complete."
    echo ""
    echo "  Launching in 3..."
    sleep 1
    echo "  2..."
    sleep 1
    echo "  1..."
    sleep 1
    echo ""

    ./target/release/code_arcade

    echo ""
    echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Thanks for playing Terminal of Babel."
    echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
else
    echo ""
    echo "  Build failed. Check errors above."
    echo "  Try: cargo clean && cargo build --release"
    exit 1
fi
