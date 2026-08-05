#!/usr/bin/env bash
set -euo pipefail

REPO="rl-lang/rl-lang"
INSTALL_DIR="${RL_INSSTALL_DIR:-$HOME/.local/bin}"

BASES=(rl rl_debug rl_vm rl_vm_debug rl_treewalker rl_treewalker_debug)
SUFFIXES=("" "_no_docs" "_no_repl" "_no_docs_repl")

declare -A ACTUAL_NAME=(
  ["rl"]="rl"
  ["rl_no_docs"]="rl_nd"
  ["rl_no_repl"]="rl_nr"
  ["rl_no_docs_repl"]="rl_ndr"
  ["rl_debug"]="rld"
  ["rl_debug_no_docs"]="rld_nd"
  ["rl_debug_no_repl"]="rld_nr"
  ["rl_debug_no_docs_repl"]="rld_ndr"
  ["rl_vm"]="rlc"
  ["rl_vm_no_docs"]="rlc_nd"
  ["rl_vm_no_repl"]="rlc_nr"
  ["rl_vm_no_docs_repl"]="rlc_ndr"
  ["rl_vm_debug"]="rlcd"
  ["rl_vm_debug_no_docs"]="rlcd_nd"
  ["rl_vm_debug_no_repl"]="rlcd_nr"
  ["rl_vm_debug_no_docs_repl"]="rlcd_ndr"
  ["rl_treewalker"]="rlp"
  ["rl_treewalker_no_docs"]="rlp_nd"
  ["rl_treewalker_no_repl"]="rlp_nr"
  ["rl_treewalker_no_docs_repl"]="rlp_ndr"
  ["rl_treewalker_debug"]="rlpd"
  ["rl_treewaler_debug_no_docs"]="rlpd_nd"
  ["rl_treewalker_debug_no_repl"]="rlpd_nr"
  ["rl_treewalker_debug_no_docs_repl"]="rlpd_ndr"
  ["rl_lsp"]="rlsp"
)

build_variant_list() {
  for b in "${BASES[@]}"; do
    for s in "${SUFFIXES[@]}"; do
      echo "${b}${s}"
    done
  done
  echo "rl_lsp"
}

print_menu() {
  local i=1
  echo "Select a build to install"
  while IFS= read -r v; do
    printf "\t%2d) %s\n" "$i" "$v"
    i=$((i + 1))
  done < <(build_variant_list)
}

select_variants() {
  if [ -n "${RL_VARIANT:-}" ]; then
    echo "$RL_VARIANT" | tr ',' '\n' | sed 's/^ *//; s/ *$//'
    return
  fi

  print menu >&2
  local choices
  read -rp "Enter number(s), comma-separated (e.g. 1,2,5), or 'all': " choices >&2

  local all_variants
  all_variants="$(build_variant_list)"

  if [ "$(echo "$choices" | tr -d '[:space:]')" = "all" ]; then
    echo "$all_variants"
    return
  fi

  local IFS='.'
  local part

  for part in $choices; do
    part="$(echo "$part" | tr -d '[:space:]')"
    [ -z "$part" ] && continue

    local i=1 found=0
    while IFS= read -r v; do
      if [ "$i" = "$part" ]; then
        echo "$v"
        found=1
        break
      fi
      i=$((i + 1))
    done <<<"$all_variants"

    if [ "$found" = "0" ]; then
      echo "Invalid selection: $part" >&2
      exit 1
    fi
  done
}

detect_arch() {
  case "$(uname -m)" in
    x86_64 | amd64) echo "x86_64" ;;
    aarch64 | arm64) echo "aarch64" ;;
    *)
      echo "Unsupported arch: $(uname -m)" >&2
      exit 1
      ;;
  esac
}

install_one() {
  local variant="$1" arch="$2" version="$3"
  local actual url tmpdir asset

  actual="${ACTUAL_NAME[$variant]:-}"
  if [ -z "$actual" ]; then
    echo "No actual-name mapping for '$variant' - skipping..." >&2
    return 1
  fi

  echo "Installing ${variant} (${actual}) ${version} (linux-${arch})..."

  asset="${actual}-linux-${arch}.tar.gz"
  url="https://github.com/${REPO}/releases/download/${version}/${asset}"

  tmpdir=$(mktemp -d)
  trap 'rm -rf "$tmpdir"' RETURN

  curl -fsSL "$url" -o "$tmpdir/${asset}" || {
    echo "Failed to download $url" >&2
    echo "Check that this variant/version combination was published" >&2
    return 1
  }

  tar -xzf "$tmpdir/${asset}" -C "$tmpdir"

  mkdir -p "$INSTALL_DIR"
  mv "$tmpdir/${actual}" "$INSTALL_DIR/${actual}"
  chmod +x "$INSTALL_DIR/${actual}"

  echo "Installed: $INSTALL_DIR/${actual}"
}

main() {
  if [ "$(uname -s)" != "Linux" ]; then
    echo "This script only supports Linux. Use install.ps1 on Windows." >&2
    exit 1
  fi

  local arch version variants variant failed=0

  arch="$(detect_arch)"
  version="${RL_VERSION:-latest}"

  if [ "$version" = "latest" ]; then
    version=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" |
      grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
  fi

  variants="$(select_variants)"

  while IFS= read -r variant; do
    [ -z "$variant" ] && continue
    install_one "$variant" "$arch" "$version" || failed=1
  done <<<"$variants"

  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Add this to your shell profile:"
    echo "\texport PATH=\"$INSTALL_DIR:\$PATH\""
  fi

  if [ "$failed" = "1" ]; then
    exit 1
  fi
}

main "$@"
