#!/bin/sh
# oppsy-cli installer
# Usage: curl -fsSL https://github.com/oppsy-dev/oppsy-dev/releases/latest/download/install-cli.sh | sh

set -e

REPO="oppsy-dev/oppsy-dev"
BINARY="oppsy-cli"
INSTALL_DIR="${OPPSY_INSTALL_DIR:-/usr/local/bin}"

# ── Colours ────────────────────────────────────────────────────────────────────
if [ -t 1 ]; then
  RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[0;33m'; BOLD='\033[1m'; RESET='\033[0m'
else
  RED=''; GREEN=''; YELLOW=''; BOLD=''; RESET=''
fi

info()    { printf "${BOLD}info${RESET}  %s\n" "$1"; }
success() { printf "${GREEN}ok${RESET}    %s\n" "$1"; }
warn()    { printf "${YELLOW}warn${RESET}  %s\n" "$1"; }
error()   { printf "${RED}error${RESET} %s\n" "$1" >&2; exit 1; }

# ── Downloader ─────────────────────────────────────────────────────────────────
download() {
  url="$1"; dest="$2"
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$url" -o "$dest"
  elif command -v wget > /dev/null 2>&1; then
    wget -qO "$dest" "$url"
  else
    error "neither curl nor wget found — cannot download files"
  fi
}

download_stdout() {
  url="$1"
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$url"
  elif command -v wget > /dev/null 2>&1; then
    wget -qO- "$url"
  else
    error "neither curl nor wget found — cannot download files"
  fi
}

# ── OS detection ───────────────────────────────────────────────────────────────
detect_os() {
  os="$(uname -s)"
  case "$os" in
    Linux)  echo "linux" ;;
    Darwin) echo "darwin" ;;
    *)      error "unsupported OS: $os" ;;
  esac
}

# ── Arch detection ─────────────────────────────────────────────────────────────
detect_arch() {
  arch="$(uname -m)"
  case "$arch" in
    x86_64 | amd64)         echo "amd64" ;;
    aarch64 | arm64)        echo "arm64" ;;
    i386 | i486 | i586 | i686) echo "386" ;;
    *)                      error "unsupported architecture: $arch" ;;
  esac
}

# ── Latest version ─────────────────────────────────────────────────────────────
latest_version() {
  url="https://api.github.com/repos/${REPO}/releases/latest"
  version="$(download_stdout "$url" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"
  if [ -z "$version" ]; then
    error "could not determine latest release version"
  fi
  echo "$version"
}

# ── Main ───────────────────────────────────────────────────────────────────────
main() {
  goos="$(detect_os)"
  goarch="$(detect_arch)"

  info "detected platform: ${goos}/${goarch}"

  version="${OPPSY_VERSION:-$(latest_version)}"
  info "installing ${BINARY} ${version}"

  archive="${BINARY}_${version}_${goos}_${goarch}.tar.gz"
  url="https://github.com/${REPO}/releases/download/${version}/${archive}"

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  info "downloading ${archive}..."
  download "$url" "${tmp}/${archive}"

  info "extracting..."
  tar -xzf "${tmp}/${archive}" -C "$tmp"

  if [ ! -f "${tmp}/${BINARY}" ]; then
    error "binary '${BINARY}' not found in archive"
  fi

  # Install — try INSTALL_DIR, fall back to ~/.local/bin
  if [ -w "$INSTALL_DIR" ] || mkdir -p "$INSTALL_DIR" 2>/dev/null; then
    install -m 755 "${tmp}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
    success "installed to ${INSTALL_DIR}/${BINARY}"
  else
    warn "${INSTALL_DIR} is not writable, trying with sudo..."
    sudo install -m 755 "${tmp}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
    success "installed to ${INSTALL_DIR}/${BINARY}"
  fi

  # PATH hint
  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    warn "${INSTALL_DIR} is not in your PATH — add it to your shell profile:"
    warn "  export PATH=\"${INSTALL_DIR}:\$PATH\""
  fi

  success "${BINARY} ${version} is ready — run: ${BINARY} --help"
}

main
