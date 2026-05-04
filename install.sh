#!/bin/bash
# kimi-dotfiles interactive installer
# Usage: cd your-project && bash /path/to/kimi-dotfiles/install.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=========================================="
echo "  kimi-dotfiles installer"
echo "=========================================="
echo ""

# Detect project type
HAS_RUST=false
HAS_EXISTING=false

if [ -f "Cargo.toml" ]; then
    HAS_RUST=true
fi

if [ -f "AGENTS.md" ] || [ -f ".kimi/AGENTS.md" ]; then
    HAS_EXISTING=true
fi

echo "Detected project type:"
[ "$HAS_RUST" = true ] && echo "  ✓ Rust (Cargo.toml found)"
[ "$HAS_EXISTING" = true ] && echo "  ⚠ Existing AGENTS.md found"
[ "$HAS_RUST" = false ] && echo "  ? No Cargo.toml detected — are you in the right directory?"
echo ""

# Non-interactive mode support
TEMPLATE=""
STRICTNESS="standard"
YES=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --template)
            TEMPLATE="$2"
            shift 2
            ;;
        --strictness)
            STRICTNESS="$2"
            shift 2
            ;;
        --yes)
            YES=true
            shift
            ;;
        --help)
            echo "Usage: install.sh [--template minimal|rust-only|full] [--strictness relaxed|standard|strict] [--yes]"
            echo ""
            echo "Options:"
            echo "  --template NAME      Use template without prompting"
            echo "  --strictness LEVEL   Clippy strictness: relaxed|standard|strict (default: standard)"
            echo "  --yes                Auto-confirm overwrites (backup still created)"
            echo "  --help               Show this help"
            echo ""
            echo "Examples:"
            echo "  bash install.sh --template rust-only --strictness relaxed --yes"
            echo "  bash install.sh --template full --strictness strict"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Run: install.sh --help"
            exit 1
            ;;
    esac
done

# Validate strictness
if [[ "$STRICTNESS" != "relaxed" && "$STRICTNESS" != "standard" && "$STRICTNESS" != "strict" ]]; then
    echo "Error: --strictness must be one of: relaxed, standard, strict"
    exit 1
fi

# Ask user what they want if not provided
if [ -z "$TEMPLATE" ]; then
    echo "What do you want to install?"
    echo ""
    echo "  1) Minimal (base rules only)"
    echo "  2) Rust-specific (base + Rust)"
    echo "  3) Full (base + Rust)"
    echo ""
    read -r -p "Enter choice [1-3]: " CHOICE

    case "$CHOICE" in
        1) TEMPLATE="minimal" ;;
        2) TEMPLATE="rust-only" ;;
        3) TEMPLATE="full" ;;
        *)
            echo "Invalid choice. Exiting."
            exit 1
            ;;
    esac
fi

# Determine target path
TARGET_PATH="AGENTS.md"
if [ -f ".kimi/AGENTS.md" ]; then
    TARGET_PATH=".kimi/AGENTS.md"
fi

# Handle existing files
if [ "$HAS_EXISTING" = true ]; then
    if [ "$YES" = true ]; then
        [ -f "AGENTS.md" ] && cp "AGENTS.md" "AGENTS.md.backup.$(date +%s)"
        [ -f ".kimi/AGENTS.md" ] && cp ".kimi/AGENTS.md" ".kimi/AGENTS.md.backup.$(date +%s)"
    else
        echo ""
        echo "⚠ AGENTS.md already exists at $TARGET_PATH!"
        echo ""
        echo "  1) Overwrite (backup created)"
        echo "  2) Save as AGENTS.md.new"
        echo "  3) Abort"
        echo ""
        read -r -p "Enter choice [1-3]: " EXISTING_CHOICE

        case "$EXISTING_CHOICE" in
            1)
                [ -f "AGENTS.md" ] && cp "AGENTS.md" "AGENTS.md.backup"
                [ -f ".kimi/AGENTS.md" ] && cp ".kimi/AGENTS.md" ".kimi/AGENTS.md.backup"
                ;;
            2)
                cp "$SCRIPT_DIR/templates/$TEMPLATE/AGENTS.md" "AGENTS.md.new"
                echo "Copied to AGENTS.md.new — review and merge manually."
                exit 0
                ;;
            *)
                echo "Aborted."
                exit 0
                ;;
        esac
    fi
fi

# Copy template
echo ""
echo "Installing $TEMPLATE template to $TARGET_PATH..."
cp "$SCRIPT_DIR/templates/$TEMPLATE/AGENTS.md" "$TARGET_PATH"

# Update strictness comment in the generated AGENTS.md
sed -i.bak "s/<!-- Strictness: standard -->/<!-- Strictness: $STRICTNESS -->/" "$TARGET_PATH" && rm -f "$TARGET_PATH.bak"

# Language-specific extras
if [ "$HAS_RUST" = true ]; then
    if [ "$TEMPLATE" = "rust-only" ] || [ "$TEMPLATE" = "full" ]; then
        if [ -f ".cargo/config.toml" ]; then
            echo "  ⚠ .cargo/config.toml exists, creating backup..."
            cp ".cargo/config.toml" ".cargo/config.toml.backup.$(date +%s)"
        fi
        mkdir -p .cargo
        cp "$SCRIPT_DIR/strictness/$STRICTNESS.toml" ".cargo/config.toml"
        echo "  ✓ Created .cargo/config.toml (strictness: $STRICTNESS)"
    fi
fi

echo ""
echo "=========================================="
echo "  Installation complete!"
echo "=========================================="
echo ""
echo "Files created/modified:"
[ -f "$TARGET_PATH" ] && echo "  ✓ $TARGET_PATH"
[ -f ".cargo/config.toml" ] && echo "  ✓ .cargo/config.toml"
echo ""
echo "Next steps:"
echo "  1. Review $TARGET_PATH"
echo "  2. Add project-specific rules at the bottom"
echo "  3. Commit: git add AGENTS.md .cargo/config.toml && git commit -m 'Add kimi-dotfiles guidelines'"
echo ""
echo "Version lock:"
echo "  <!-- kimi-dotfiles: v1.3.0 -->"
