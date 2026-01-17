# 🎮 Visual UI Guide - Code Arcade

## Screen Layouts

### 1. Main Coding Screen
```
╔═══════════════════════════════════════╗
║ ⚡ CODE ARCADE ⚡ LANGUAGE ROULETTE   ║
╚═══════════════════════════════════════╝
┌─────────────────────┬──────────────────────────────────┐
│ PROBLEM             │ EDITOR - Python                  │
│                     │                                  │
│ 1. Two Sum          │   1  def two_sum(nums, target): │
│                     │   2      # Write your solution  │
│ Description:        │   3      pass                   │
│ Given an array of   │   4                             │
│ integers nums and   │   5                             │
│ an integer target,  │                                  │
│ return indices...   │                                  │
│                     │                                  │
│ Example 1:          │                                  │
│ Input: [2,7,11,15]  │                                  │
│ target = 9          │                                  │
│ Output: [0,1]       │                                  │
│                     │                                  │
│ Example 2:          │                                  │
│ ...                 │                                  │
└─────────────────────┴──────────────────────────────────┘
    ⏱ NEXT RANDOMIZE: 42s │ Cmd+S to Submit
```

### 2. Transition Screen (Language Randomization)
```
▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█
█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓
░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒
        ┌─────────────────────────┐
        │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │
        │                         │
        │  RANDOMIZING CODE...    │
        │                         │
        │  PROGRESS: 67%          │
        │                         │
        │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │
        └─────────────────────────┘
▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█
▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓░▒█▓
```

### 3. Results Screen
```
╔═══════════════════════════════════════╗
║          RESULTS SCREEN              ║
╚═══════════════════════════════════════╝

SCORE: 3/4 (75%)

═══════════════════════════════════════

✓ PASS Test #1
  Input: [2,7,11,15], 9
  Expected: [0,1]

✓ PASS Test #2
  Input: [3,2,4], 6
  Expected: [1,2]

✓ PASS Test #3
  Input: [3,3], 6
  Expected: [0,1]

✗ FAIL Test #4
  Input: [-1,-2,-3,-4,-5], -8
  Expected: [2,4]
  Got: []

═══════════════════════════════════════

Press 'R' to restart or 'Q' to quit
```

## Color Scheme

### Headers & Borders
- **Cyan** (#00FFFF): Main borders, box drawing
- **Yellow** (#FFFF00): Title "CODE ARCADE", bold emphasis
- **Magenta** (#FF00FF): "LANGUAGE ROULETTE", editor border

### Content
- **White** (#FFFFFF): Main text, code
- **Gray** (#808080): Examples, secondary info
- **Dark Gray** (#404040): Line numbers, separators

### Status Indicators
- **Green** (#00FF00): 
  - Timer > 20s remaining
  - Passed tests (✓)
  - Score > 80%

- **Yellow** (#FFFF00):
  - Timer 10-20s remaining
  - Warnings
  - Score 50-80%

- **Red** (#FF0000):
  - Timer < 10s remaining
  - Failed tests (✗)
  - Score < 50%

### Transition Effects
- **Cycling Cyan/Magenta/Blue**: Glitch animation
- **Random intensity**: Creates CRT monitor effect

## UI States Flow

```
     ┌──────────┐
     │  Start   │
     └────┬─────┘
          │
          v
     ┌──────────────┐
     │   Coding     │◄──────┐
     │  (45 secs)   │       │
     └────┬─────────┘       │
          │                 │
          │ Timer expires   │
          │     OR          │
          │  Cmd+S pressed  │
          │                 │
          v                 │
     ┌──────────────┐       │
     │ Transitioning│       │
     │  (2 secs)    │       │
     └────┬─────────┘       │
          │                 │
          │ If timer:       │
          │  randomize      │
          │  language       │
          └─────────────────┘
          │
          │ If Cmd+S:
          │  run tests
          │
          v
     ┌──────────────┐
     │   Results    │
     │ (wait user)  │
     └────┬─────────┘
          │
          │ Press R
          │
          v
     ┌──────────┐
     │ Restart  │
     └──────────┘
```

## Animation Details

### Timer Countdown
- Updates every second
- Color changes at thresholds:
  - 45-21s: Green
  - 20-11s: Yellow  
  - 10-0s: Red (urgent!)

### Transition Glitch
- 2-second total duration
- Progress: 0% → 100%
- 100ms frame updates
- Random character placement
- Color shifts based on screen position
- Characters used: █ ▓ ▒ ░ ▀ ▄ ▌ ▐

### Cursor Behavior (Future Enhancement)
- Currently hidden during runtime
- Could add blinking cursor in editor
- Could highlight current line

## Keyboard Input Handling

All handled in raw mode:
- Character input → Direct insertion
- Control sequences → Intercepted
- Platform-specific mods → Normalized (Cmd/Ctrl both work)

## Terminal Requirements

- **Minimum size**: 80x24 characters
- **Color support**: 256 colors recommended
- **UTF-8 support**: For box drawing characters
- **Platforms**: macOS, Linux, Windows Terminal

## Accessibility Notes

Current implementation:
- Pure text-based (screen reader compatible structure)
- High contrast color scheme
- Clear state indicators
- Keyboard-only navigation

Future improvements:
- Configurable colors
- Sound cues option
- Font size hints
- Reduced motion mode

---

**The arcade aesthetic isn't just for show - it makes the coding challenge feel like a game! 🎮**
