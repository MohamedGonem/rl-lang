#!/usr/bin/env bash
set -euo pipefail

TARGET="$1"
PLATFORM="$2"
ARCH="$3"
OUT_DIR="$4"
VARIANT_LIST="${5:-}"

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

declare -A ENGINE_FEATURES=(
  ["rl"]="treewalker,vm"
  ["rl_vm"]="vm"
  ["rl_treewalker"]="treewalker"
)

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
  ["rl_treewalker_debug_no_docs"]="rlpd_nd"
  ["rl_treewalker_debug_no_repl"]="rlpd_nr"
  ["rl_treewalker_debug_no_docs_repl"]="rlpd_ndr"
  ["rl_lsp"]="rlsp"
)

BASES=(rl rl_debug rl_vm rl_vm_debug rl_treewalker rl_treewalker_debug)
SUFFIXES=("" "_no_docs" "_no_repl" "_no_docs_repl")

build_variant_list() {
  for b in "${BASES[@]}"; do
    for s in "${SUFFIXES[@]}"; do
      echo "${b}${s}"
    done
  done
  echo "rl_lsp"
}

features_for() {
  local variant="$1"

  if [ "$variant" = "rl_lsp" ]; then
    echo "lsp"
    return
  fi

  local base="$variant"
  local no_docs=0 no_repl=0

  if [[ "$base" == *_no_docs_repl ]]; then
    no_docs=1
    no_repl=1
    base="${base%_no_docs_repl}"
  elif [[ "$base" == *_no_docs ]]; then
    no_docs=1
    base="${base%_no_docs}"
  elif [[ "$base" == *_no_repl ]]; then
    no_repl=1
    base="${base%_no_repl}"
  fi

  local debug=0
  if [[ "$base" == *_debug ]]; then
    debug=1
    base="${base%_debug}"
  fi

  local feats="${ENGINE_FEATURES[$base]:-}"
  if [ -z "$feats" ]; then
    echo "unknown base '$base' derived from variant '$variant'" >&2
    exit 1
  fi

  [ "$no_docs" = "0" ] && feats="${feats},docs"
  [ "$no_repl" = "0" ] && feats="${feats},repl"
  [ "$debug" = "1" ] && feats="${feats},debug"

  echo "$feats"
}

package_linux() {
  local actual="$1" bin_path="$2"
  local stage
  stage=$(mktemp -d)
  cp "$bin_path" "$stage/${actual}"
  chmod +x "$stage/${actual}"
  tar -czf "$OUT_DIR/${actual}-linux-${ARCH}.tar.gz" -C "$stage" "${actual}"
  rm -rf "$stage"
}

package_windows() {
  local actual="$1" bin_path="$2"
  local stage
  stage=$(mktemp -d)
  cp "$bin_path" "$stage/${actual}.exe"
  (cd "$stage" && zip -q "$OUT_DIR/${actual}-windows-${ARCH}.zip" "${actual}.exe")
  rm -rf "$stage"
}

main() {
  local variant actual feats bin_src

  if [ -n "$VARIANT_LIST" ]; then
    while IFS= read -r variant; do
      [ -n "$variant" ] || continue
      build_one "$variant" "$TARGET" "$PLATFORM" "$ARCH" "$OUT_DIR"
    done <<<"$(printf '%s\n' "$VARIANT_LIST" | tr ',' '\n' | sed 's/^ *//; s/ *$//')"
  else
    while IFS= read -r variant; do
      build_one "$variant" "$TARGET" "$PLATFORM" "$ARCH" "$OUT_DIR"
    done < <(build_variant_list)
  fi
}

build_one() {
  local variant="$1" target="$2" platform="$3" arch="$4" out_dir="$5"
  local actual feats bin_src

  actual="${ACTUAL_NAME[$variant]:-}"
  if [ -z "$actual" ]; then
    echo "unknown variant '$variant'" >&2
    exit 1
  fi
  feats="$(features_for "$variant")"

  echo "=== Building ${variant} (${actual}) [features: ${feats}] ==="
  cargo build --release --no-default-features --features "$feats" \
    --target "$target" -p rl-cli

  if [ "$platform" = "windows" ]; then
    bin_src="target/${target}/release/rl.exe"
  else
    bin_src="target/${target}/release/rl"
  fi

  if [ ! -f "$bin_src" ]; then
    echo "expected binary not found at $bin_src" >&2
    exit 1
  fi

  if [ "$platform" = "windows" ]; then
    package_windows "$actual" "$bin_src"
  else
    package_linux "$actual" "$bin_src"
  fi
}

main
