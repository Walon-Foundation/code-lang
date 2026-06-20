#!/usr/bin/env sh
set -e

REPO="Walon-Foundation/code-lang"
INSTALL_DIR="${HOME}/.code-lang/bin"

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

# ── fetch release info ────────────────────────────────────────────────────────
RELEASE_JSON=$(${FETCH} "https://api.github.com/repos/${REPO}/releases/latest")

VERSION=$(echo "$RELEASE_JSON" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "error: could not determine latest release version"
  exit 1
fi

echo "installing code-lang ${VERSION} for ${TARGET}"
echo ""

# ── helper: download one archive and extract a named binary ──────────────────
install_bin() {
  BIN_NAME="$1"
  # match the archive for this binary and target, skip checksums/manifests/installers
  URL=$(echo "$RELEASE_JSON" \
    | grep '"browser_download_url"' \
    | grep "${BIN_NAME}" \
    | grep "${TARGET}" \
    | grep -v 'installer\|\.sha256\|dist-manifest' \
    | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/' \
    | head -1)

  if [ -z "$URL" ]; then
    echo "warning: no release asset found for '${BIN_NAME}' on ${TARGET}, skipping"
    return
  fi

  ARCHIVE=$(basename "$URL")
  TMP=$(mktemp -d)
  trap 'rm -rf "$TMP"' EXIT

  echo "downloading ${BIN_NAME}..."
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$URL" -o "${TMP}/${ARCHIVE}"
  else
    wget -q "$URL" -O "${TMP}/${ARCHIVE}"
  fi

  case "$ARCHIVE" in
    *.tar.gz) tar -xzf "${TMP}/${ARCHIVE}" -C "$TMP" ;;
    *.tar.xz) tar -xJf "${TMP}/${ARCHIVE}" -C "$TMP" ;;
    *.zip)    unzip -q  "${TMP}/${ARCHIVE}" -d "$TMP" ;;
    *)
      echo "error: unknown archive format: ${ARCHIVE}"
      exit 1
      ;;
  esac

  BINARY=$(find "$TMP" -name "$BIN_NAME" -type f | head -1)
  if [ -z "$BINARY" ]; then
    echo "error: could not find '${BIN_NAME}' in the archive"
    exit 1
  fi

  mv "$BINARY" "${INSTALL_DIR}/${BIN_NAME}"
  chmod +x "${INSTALL_DIR}/${BIN_NAME}"
  echo "installed: ${INSTALL_DIR}/${BIN_NAME}"
}

# ── install ───────────────────────────────────────────────────────────────────
mkdir -p "$INSTALL_DIR"
install_bin "code-lang"
install_bin "code-lang-fmt"

echo ""

# ── PATH check ────────────────────────────────────────────────────────────────
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*)
    ;;
  *)
    echo "note: ${INSTALL_DIR} is not in your PATH"
    echo "add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo '  export PATH="${HOME}/.code-lang/bin:${PATH}"'
    echo ""
    ;;
esac

echo "done — run: code-lang --version"
