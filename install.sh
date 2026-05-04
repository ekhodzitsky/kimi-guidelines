#!/bin/bash
# kimi-dotfiles interactive installer
# Usage: cd your-project && bash /path/to/kimi-dotfiles/install.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=========================================="
echo "  kimi-dotfiles installer"
echo "=========================================="
echo ""

# Detect project type
HAS_RUST=false
HAS_SWIFT=false
HAS_EXISTING=false

if [ -f "Cargo.toml" ] || [ -d "src" ]; then
    HAS_RUST=true
fi

if [ -f "Package.swift" ] || [ -d "Sources" ]; then
    HAS_SWIFT=true
fi

if [ -f "AGENTS.md" ] || [ -f ".kimi/AGENTS.md" ]; then
    HAS_EXISTING=true
fi

echo "Detected project type:"
[ "$HAS_RUST" = true ] && echo "  ✓ Rust"
[ "$HAS_SWIFT" = true ] && echo "  ✓ Swift"
[ "$HAS_EXISTING" = true ] && echo "  ⚠ Existing AGENTS.md found"
[ "$HAS_RUST" = false ] && [ "$HAS_SWIFT" = false ] && echo "  ? Unknown (no Cargo.toml or Package.swift)"
echo ""

# Ask user what they want
echo "What do you want to install?"
echo ""
echo "  1) Minimal (base rules only)"
echo "  2) Rust-specific (base + Rust)"
echo "  3) Swift-specific (base + Swift)"
echo "  4) Full (base + Rust + Swift)"
echo "  5) Just copy templates for manual editing"
echo ""
read -p "Enter choice [1-5]: " CHOICE

case "$CHOICE" in
    1)
        TEMPLATE="minimal"
        ;;
    2)
        TEMPLATE="rust-only"
        ;;
    3)
        TEMPLATE="swift-only"
        ;;
    4)
        TEMPLATE="full"
        ;;
    5)
        echo "Copying templates to .kimi/dotfiles/ ..."
        mkdir -p .kimi/dotfiles
        cp -r "$SCRIPT_DIR/templates/"* .kimi/dotfiles/
        cp -r "$SCRIPT_DIR/languages/"* .kimi/dotfiles/
        echo "Done. Templates are in .kimi/dotfiles/"
        exit 0
        ;;
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

# Handle existing files
if [ "$HAS_EXISTING" = true ]; then
    echo ""
    echo "⚠ AGENTS.md already exists!"
    echo ""
    echo "  1) Overwrite (backup as AGENTS.md.backup)"
    echo "  2) Merge manually later (copy to AGENTS.md.new)"
    echo "  3) Abort"
    echo ""
    read -p "Enter choice [1-3]: " EXISTING_CHOICE

    case "$EXISTING_CHOICE" in
        1)
            [ -f "AGENTS.md" ] && cp AGENTS.md AGENTS.md.backup
            [ -f ".kimi/AGENTS.md" ] && cp .kimi/AGENTS.md .kimi/AGENTS.md.backup
            ;;
        2)
            cp "$SCRIPT_DIR/templates/$TEMPLATE/AGENTS.md" AGENTS.md.new
            echo "Copied to AGENTS.md.new — review and merge manually."
            exit 0
            ;;
        *)
            echo "Aborted."
            exit 0
            ;;
    esac
fi

# Copy template
echo ""
echo "Installing $TEMPLATE template..."
cp "$SCRIPT_DIR/templates/$TEMPLATE/AGENTS.md" AGENTS.md

# Language-specific extras
if [ "$TEMPLATE" = "rust-only" ] || [ "$TEMPLATE" = "full" ]; then
    if [ "$HAS_RUST" = true ]; then
        echo "  ✓ Copying Rust clippy config example..."
        mkdir -p .cargo
        cat > .cargo/config.toml << 'EOF'
[lints.clippy]
all = "deny"
pedantic = "warn"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
EOF
    fi
fi

if [ "$TEMPLATE" = "swift-only" ] || [ "$TEMPLATE" = "full" ]; then
    if [ "$HAS_SWIFT" = true ]; then
        echo "  ✓ Copying SwiftLint config example..."
        cat > .swiftlint.yml << 'EOF'
disabled_rules:
  - force_try
  - force_cast
opt_in_rules:
  - empty_count
  - explicit_self
EOF
    fi
fi

echo ""
echo "=========================================="
echo "  Installation complete!"
echo "=========================================="
echo ""
echo "Files created:"
ls -la AGENTS.md 2>/dev/null
ls -la .cargo/config.toml 2>/dev/null
ls -la .swiftlint.yml 2>/dev/null
echo ""
echo "Next steps:"
echo "  1. Review AGENTS.md"
echo "  2. Add project-specific rules at the bottom"
echo "  3. Commit: git add AGENTS.md && git commit -m 'Add kimi-dotfiles guidelines'"
echo ""
echo "Version lock:"
echo "  <!-- kimi-dotfiles: v1.0.0 -->"
