#!/usr/bin/env bash
# Dx Shell Enhancement Installation Script
# This script sets up enhanced shell features for Dx CLI

set -e

DX_SHELL_DIR="${HOME}/.dx/shell"
DX_CONFIG_DIR="${HOME}/.dx"

echo "🚀 Installing Dx Shell Enhancements..."

# Create directories
mkdir -p "${DX_SHELL_DIR}"
mkdir -p "${DX_CONFIG_DIR}/history"

# Detect shell
CURRENT_SHELL=$(basename "$SHELL")
SHELL_RC=""

case "$CURRENT_SHELL" in
    bash)
        SHELL_RC="${HOME}/.bashrc"
        ;;
    zsh)
        SHELL_RC="${HOME}/.zshrc"
        ;;
    fish)
        SHELL_RC="${HOME}/.config/fish/config.fish"
        ;;
    *)
        echo "⚠️  Unsupported shell: $CURRENT_SHELL"
        echo "   Supported shells: bash, zsh, fish"
        exit 1
        ;;
esac

# Copy enhancement scripts
cp "$(dirname "$0")/dx-shell-init.sh" "${DX_SHELL_DIR}/"
cp "$(dirname "$0")/dx-shell-commands.sh" "${DX_SHELL_DIR}/"
cp "$(dirname "$0")/dx-shell-memory.sh" "${DX_SHELL_DIR}/"

# Add initialization to shell rc file
if ! grep -q "# Dx Shell Enhancements" "$SHELL_RC" 2>/dev/null; then
    cat >> "$SHELL_RC" << 'EOF'

# Dx Shell Enhancements
if [ -f "$HOME/.dx/shell/dx-shell-init.sh" ]; then
    source "$HOME/.dx/shell/dx-shell-init.sh"
fi
EOF
    echo "✓ Added Dx initialization to $SHELL_RC"
else
    echo "✓ Dx already initialized in $SHELL_RC"
fi

# Initialize memory database
if [ ! -f "${DX_CONFIG_DIR}/shell.db" ]; then
    touch "${DX_CONFIG_DIR}/shell.db"
    echo "✓ Created shell memory database"
fi

echo ""
echo "✨ Dx Shell Enhancements installed successfully!"
echo ""
echo "Features enabled:"
echo "  • Enhanced ls with file type icons and sizes"
echo "  • Command help hints as you type"
echo "  • Global fuzzy autocomplete"
echo "  • Smart command suggestions for typos"
echo "  • Persistent command history and memory"
echo ""
echo "Please restart your shell or run:"
echo "  source $SHELL_RC"
