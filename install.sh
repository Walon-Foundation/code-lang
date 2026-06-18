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

# ── fetch tool ───────────────────────────────────────────────────────────────
if command -v curl > /dev/null 2>&1; then
  FETCH="curl -fsSL"
elif command -v wget > /dev/null 2>&1; then
  FETCH="wget -qO-"
else
  echo "error: curl or wget is required"
  exit 1
fi

# ── find the asset URL for our target from the latest release ────────────────
# Queries the GitHub API and picks the browser_download_url that contains our
# target triple, excluding installer scripts, checksums, and manifests.
RELEASE_JSON=$(${FETCH} "https://api.github.com/repos/${REPO}/releases/latest")

VERSION=$(echo "$RELEASE_JSON" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "error: could not determine latest release version"
  exit 1
fi

URL=$(echo "$RELEASE_JSON" \
  | grep '"browser_download_url"' \
  | grep "${TARGET}" \
  | grep -v 'installer\|\.sha256\|dist-manifest' \
  | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/' \
  | head -1)

if [ -z "$URL" ]; then
  echo "error: no release asset found for ${TARGET}"
  echo "check https://github.com/${REPO}/releases for available targets"
  exit 1
fi

# derive archive filename from the URL
ARCHIVE=$(basename "$URL")

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

# ── extract (handles both .tar.gz and .tar.xz) ───────────────────────────────
case "$ARCHIVE" in
  *.tar.gz)  tar -xzf "${TMP}/${ARCHIVE}" -C "$TMP" ;;
  *.tar.xz)  tar -xJf "${TMP}/${ARCHIVE}" -C "$TMP" ;;
  *.zip)     unzip -q "${TMP}/${ARCHIVE}" -d "$TMP" ;;
  *)
    echo "error: unknown archive format: ${ARCHIVE}"
    exit 1
    ;;
esac

# find the binary (cargo-dist nests it in a subdirectory)
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
