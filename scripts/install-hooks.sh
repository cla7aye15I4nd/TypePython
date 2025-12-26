#!/bin/bash
# Install git hooks for the TypePython project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

echo "Installing git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Install pre-commit hook
if [ -f "$GIT_HOOKS_DIR/pre-commit" ]; then
    echo "Warning: pre-commit hook already exists. Creating backup..."
    cp "$GIT_HOOKS_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit.backup"
fi

cp "$SCRIPT_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
chmod +x "$GIT_HOOKS_DIR/pre-commit"

echo "✓ Pre-commit hook installed successfully!"
echo ""
echo "The hook will run automatically before each commit to check:"
echo "  - Code formatting (cargo fmt)"
echo "  - Linting (cargo clippy)"
echo "  - Build (cargo build)"
echo "  - Tests (cargo test)"
echo ""
echo "To skip the hook for a specific commit, use: git commit --no-verify"