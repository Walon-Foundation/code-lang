#!/usr/bin/env sh
set -e

REPO="Walon-Foundation/code-lang"
BIN="code-lang"
INSTALL_DIR="${HOME}/.local/bin"

# ── OS detection ────────────────────────────────────────────────────────────
OS=$(uname -s)
case "$OS" in
  Linux)  OS_TARGET="unknown-linux-gnu" ;;
  Darwin) OS_TARGET="apple-darwin" ;;
  *)
    echo "error: unsupported OS: $OS"
    exit 1
    ;;
esac

# ── arch detection ──────────────────────────────────────────────────────────
ARCH=$(uname -m)
case "$ARCH" in
  x86_64 | amd64)  ARCH_TARGET="x86_64" ;;
  aarch64 | arm64) ARCH_TARGET="aarch64" ;;
  *)
    echo "error: unsupported architecture: $ARCH"
    exit 1
    ;;
esac

TARGET="${ARCH_TARGET}-${OS_TARGET}"

# ── latest version from GitHub ───────────────────────────────────────────────
if command -v curl > /dev/null 2>&1; then
  FETCH="curl -fsSL"
elif command -v wget > /dev/null 2>&1; then
  FETCH="wget -qO-"
else
  echo "error: curl or wget is required"
  exit 1
fi

VERSION=$(${FETCH} "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "error: could not determine latest release version"
  exit 1
fi

ARCHIVE="${BIN}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

echo "installing code-lang ${VERSION} for ${TARGET}"
echo "from: ${URL}"
echo ""

# ── download ─────────────────────────────────────────────────────────────────
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

if command -v curl > /dev/null 2>&1; then
  curl -fsSL "$URL" -o "${TMP}/${ARCHIVE}"
else
  wget -q "$URL" -O "${TMP}/${ARCHIVE}"
fi

# ── extract ───────────────────────────────────────────────────────────────────
tar -xzf "${TMP}/${ARCHIVE}" -C "$TMP"

# find the binary (cargo-dist may nest it in a subdirectory)
BINARY=$(find "$TMP" -name "$BIN" -type f | head -1)
if [ -z "$BINARY" ]; then
  echo "error: could not find binary '${BIN}' in the archive"
  exit 1
fi

# ── install ───────────────────────────────────────────────────────────────────
mkdir -p "$INSTALL_DIR"
mv "$BINARY" "${INSTALL_DIR}/${BIN}"
chmod +x "${INSTALL_DIR}/${BIN}"

echo "installed: ${INSTALL_DIR}/${BIN}"

# ── PATH check ────────────────────────────────────────────────────────────────
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*)
    ;;
  *)
    echo ""
    echo "note: ${INSTALL_DIR} is not in your PATH"
    echo "add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo '  export PATH="${HOME}/.local/bin:${PATH}"'
    echo ""
    ;;
esac

echo "done — run: code-lang --version"
