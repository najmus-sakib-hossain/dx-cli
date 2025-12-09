#!/usr/bin/env bash
# Quick Start Guide for Dx Shell Enhancements
# Run this script to see shell features in action

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  🐚 Dx Shell Enhancements - Quick Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if installed
if [ ! -f "$HOME/.dx/shell/dx-shell-init.sh" ]; then
    echo "❌ Shell enhancements not installed!"
    echo ""
    echo "Install with:"
    echo "  dx shell --enable"
    echo "  bash ~/.dx/shell/install.sh"
    exit 1
fi

echo "✓ Shell enhancements detected!"
echo ""

# Demo 1: Enhanced ls
echo "━━━ 1. Enhanced ls Command ━━━"
echo ""
echo "Standard ls vs Enhanced ls:"
echo ""
echo "$ ls"
ls --color=auto 2>/dev/null | head -5
echo ""

# Demo 2: Command Help
echo "━━━ 2. Command Help Hints ━━━"
echo ""
echo "Try: git <Ctrl+H>"
echo "Shows common git commands instantly!"
echo ""

# Demo 3: Smart Suggestions
echo "━━━ 3. Smart Command Suggestions ━━━"
echo ""
echo "Type wrong command:"
echo "$ gti status"
echo ""
echo "Get suggestions:"
echo "  ✗ Command not found: gti"
echo "  ● Did you mean: git"
echo ""

# Demo 4: Memory
echo "━━━ 4. Persistent Memory ━━━"
echo ""
echo "Commands to try:"
echo ""
echo "  dx-freq         # Show most used commands"
echo "  dx-local        # Show commands in current dir"
echo "  dx-suggest      # Context-aware suggestions"
echo "  dx-stats git    # Success rate for 'git'"
echo ""

# Demo 5: Quick Navigation
echo "━━━ 5. Quick Directory Jump ━━━"
echo ""
echo "  j <pattern>     # Jump to frequent directory"
echo "  ..              # cd .."
echo "  ...             # cd ../.."
echo ""

# Demo 6: Aliases
echo "━━━ 6. Helpful Aliases ━━━"
echo ""
echo "  ll              # ls -lah (detailed list)"
echo "  h git           # Search history for 'git'"
echo "  dxui            # dx ui"
echo "  dxchat          # dx chat"
echo "  dxtui           # dx tui"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 Full documentation:"
echo "   https://docs.dx.dev/shell-enhancements"
echo ""
echo "🎯 Get started:"
echo "   source ~/.dx/shell/dx-shell-init.sh"
echo "   # Or restart your shell"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
