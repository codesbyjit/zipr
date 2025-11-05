#!/bin/bash
# 🦀 zipr — Zero-hassle setup script (install + update + uninstall)
# Works fully without sudo, installs in ~/.local/bin

set -e

ZIPR_DIR="$HOME/.zipr"
BIN_DIR="$HOME/.local/bin"
BIN_PATH="$BIN_DIR/zipr"
REPO_URL="https://github.com/codesbyjit/zipr"
LATEST_BIN_URL="https://github.com/codesbyjit/zipr/releases/latest/download/zipr-linux"
PROFILE_FILE="$HOME/.bashrc" # Change to .zshrc if needed

banner() {
cat <<'EOF'
▒███████▒ ██▓ ██▓███   ██▀███  
▒ ▒ ▒ ▄▀░▓██▒▓██░  ██▒▓██ ▒ ██▒
░ ▒ ▄▀▒░ ▒██▒▓██░ ██▓▒▓██ ░▄█ ▒
  ▄▀▒   ░░██░▒██▄█▓▒ ▒▒██▀▀█▄  
▒███████▒░██░▒██▒ ░  ░░██▓ ▒██▒
░▒▒ ▓░▒░▒░▓  ▒▓▒░ ░  ░░ ▒▓ ░▒▓░
░░▒ ▒ ░ ▒ ▒ ░░▒ ░       ░▒ ░ ▒░
░ ░ ░ ░ ░ ▒ ░░░         ░░   ░ 
  ░ ░     ░              ░     
░                              
EOF
echo "⚡ zipr installer — smaller, quicker, smarter than zip."
echo
}

add_to_path() {
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$PROFILE_FILE"
        echo "✅ Added $BIN_DIR to PATH in $PROFILE_FILE"
        export PATH="$PATH:$BIN_DIR"
    fi
}

install_zipr() {
    echo "📦 Installing zipr..."
    mkdir -p "$BIN_DIR"
    add_to_path

    if command -v cargo >/dev/null 2>&1; then
        echo "🦀 Cargo detected — building locally from current directory..."
        cargo build --release
        cp target/release/zipr "$BIN_PATH"
        echo "✅ Built and installed zipr locally at $BIN_PATH"
    else
        echo "⚙️ Cargo not found — installing prebuilt binary..."
        curl -L "$LATEST_BIN_URL" -o "$BIN_PATH"
        chmod +x "$BIN_PATH"
        echo "✅ Installed prebuilt zipr binary at $BIN_PATH"
    fi

    echo "✨ Type 'zipr --help' to get started."
}

update_zipr() {
    echo "⬆️ Updating zipr..."
    if command -v cargo >/dev/null 2>&1 && [ -f "Cargo.toml" ]; then
        echo "🔁 Rebuilding locally..."
        cargo build --release
        cp target/release/zipr "$BIN_PATH"
        echo "✅ zipr rebuilt and updated!"
    else
        echo "⬇️ Fetching latest prebuilt binary..."
        curl -L "$LATEST_BIN_URL" -o "$BIN_PATH"
        chmod +x "$BIN_PATH"
        echo "✅ zipr updated successfully!"
    fi
}

uninstall_zipr() {
    echo "🚮 Uninstalling zipr..."
    rm -f "$BIN_PATH"
    echo "✅ zipr removed successfully!"
}

show_help() {
cat <<EOF
🦀 zipr — Setup Script

Usage:
  ./zipr-setup.sh [COMMAND]

Commands:
  install     Install zipr (uses local Cargo build if available)
  update      Update zipr (rebuild or download)
  uninstall   Remove zipr completely
  help        Show this help message

Examples:
  ./zipr-setup.sh install
  ./zipr-setup.sh update
  ./zipr-setup.sh uninstall
EOF
}

main() {
    banner
    case "$1" in
        install) install_zipr ;;
        update) update_zipr ;;
        uninstall) uninstall_zipr ;;
        help|*) show_help ;;
    esac
}

main "$@"
