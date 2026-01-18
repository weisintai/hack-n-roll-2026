#!/bin/bash
# Terminal of Babel - Start Script

clear
echo ""
echo "  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "  ┃        TERMINAL  of  BABEL          ┃"
echo "  ┃    Code in any language. Survive.   ┃"
echo "  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo ""
echo "  Controls"
echo "  ────────────────────────────"
echo "    ^S  Submit solution"
echo "    ^R  New problem"
echo "    ^C  Compile and run"
echo "    ^Q  Quit"
echo ""
echo "  The language changes every 15 seconds."
echo ""
echo "  Press any key to begin..."
read -n 1

cargo run --release
