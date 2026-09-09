#!/usr/bin/env bash
set -euo pipefail

TARGET="$1"
PLATFORM="$2"
ARCH="$3"
OUT_DIR="$4"
VARIANT_LIST="${5:-}"

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

# Build profile: "release", "nightly", or "dev-release"
# Set via RL_BUILD_PROFILE env var, or auto-detect from variant name.
BUILD_PROFILE="${RL_BUILD_PROFILE:-auto}"

# Parallel jobs: number of variants to build simultaneously.
# Set via RL_PARALLEL_JOBS, or auto-detect from nproc.
PARALLEL_JOBS="${RL_PARALLEL_JOBS:-}"

BASES=(rl rl_debug rl_vm rl_vm_debug)
SUFFIXES=("" "_no_docs" "_no_repl" "_no_docs_repl")

build_variant_list() {
  for b in "${BASES[@]}"; do
    for s in "${SUFFIXES[@]}"; do
      echo "${b}${s}"
    done
  done
  echo "rl_lsp"
}

# bash 3.2 (macOS default) doesn't support declare -A, so use case statements.

actual_name_for() {
  case "$1" in
    rl)                          echo "rl" ;;
    rl_no_docs)                  echo "rl_nd" ;;
    rl_no_repl)                  echo "rl_nr" ;;
    rl_no_docs_repl)             echo "rl_ndr" ;;
    rl_debug)                    echo "rld" ;;
    rl_debug_no_docs)            echo "rld_nd" ;;
    rl_debug_no_repl)            echo "rld_nr" ;;
    rl_debug_no_docs_repl)       echo "rld_ndr" ;;
    rl_vm)                       echo "rlc" ;;
    rl_vm_no_docs)               echo "rlc_nd" ;;
    rl_vm_no_repl)               echo "rlc_nr" ;;
    rl_vm_no_docs_repl)          echo "rlc_ndr" ;;
    rl_vm_debug)                 echo "rlcd" ;;
    rl_vm_debug_no_docs)         echo "rlcd_nd" ;;
    rl_vm_debug_no_repl)         echo "rlcd_nr" ;;
    rl_vm_debug_no_docs_repl)    echo "rlcd_ndr" ;;
    rl_lsp)                      echo "rlsp" ;;
    *)                           echo "" ;;
  esac
}

engine_features_for() {
  case "$1" in
    rl|rl_vm)  echo "vm" ;;
    *)         echo "" ;;
  esac
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

  local feats
  feats="$(engine_features_for "$base")"
  if [ -z "$feats" ]; then
    echo "unknown base '$base' derived from variant '$variant'" >&2
    exit 1
  fi

  [ "$no_docs" = "0" ] && feats="${feats},docs,docs-tui"
  [ "$no_repl" = "0" ] && feats="${feats},repl"
  [ "$debug" = "1" ] && feats="${feats},debug"

  echo "$feats"
}

# Determine the cargo profile for a given variant.
# Auto-detect: debug variants -> dev-release, nightly env -> nightly, else release.
profile_for() {
  local variant="$1"

  case "$BUILD_PROFILE" in
    release|nightly|dev-release)
      echo "$BUILD_PROFILE"
      return
      ;;
  esac

  # Auto-detect from variant name
  if [[ "$variant" == *"_debug"* ]]; then
    echo "dev-release"
  else
    echo "release"
  fi
}

cargo_profile_flag() {
  local profile="$1"
  case "$profile" in
    release)    echo "--release" ;;
    nightly)    echo "--profile nightly" ;;
    dev-release) echo "--profile dev-release" ;;
    *)          echo "--release" ;;
  esac
}

# Map a profile name to the target directory subfolder cargo uses.
cargo_profile_dir() {
  local profile="$1"
  case "$profile" in
    release)     echo "release" ;;
    nightly)     echo "nightly" ;;
    dev-release) echo "dev-release" ;;
    *)           echo "release" ;;
  esac
}

package_elf() {
  local actual="$1" bin_path="$2" label="$3"
  local stage
  stage=$(mktemp -d)
  cp "$bin_path" "$stage/${actual}"
  chmod +x "$stage/${actual}"
  tar -czf "$OUT_DIR/${actual}-${label}-${ARCH}.tar.gz" -C "$stage" "${actual}"
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

build_one() {
  local variant="$1" target="$2" platform="$3" arch="$4" out_dir="$5"
  local actual feats bin_src profile profile_flag profile_dir

  actual="$(actual_name_for "$variant")"
  if [ -z "$actual" ]; then
    echo "unknown variant '$variant'" >&2
    return 1
  fi
  feats="$(features_for "$variant")"
  profile="$(profile_for "$variant")"
  profile_flag="$(cargo_profile_flag "$profile")"
  profile_dir="$(cargo_profile_dir "$profile")"

  echo "=== Building ${variant} (${actual}) [features: ${feats}, profile: ${profile}] ==="
  # shellcheck disable=SC2086
  cargo build $profile_flag --no-default-features --features "$feats" \
    --target "$target" -p rl-cli

  if [ "$platform" = "windows" ]; then
    bin_src="target/${target}/${profile_dir}/rl.exe"
  else
    bin_src="target/${target}/${profile_dir}/rl"
  fi

  if [ ! -f "$bin_src" ]; then
    echo "expected binary not found at $bin_src" >&2
    return 1
  fi

  if [ "$platform" = "windows" ]; then
    package_windows "$actual" "$bin_src"
  elif [ "$platform" = "android" ]; then
    package_elf "$actual" "$bin_src" "android"
  else
    package_elf "$actual" "$bin_src" "$platform"
  fi

  echo "=== Done: ${variant} (${actual}) ==="
}

main() {
  local variants

  if [ -n "$VARIANT_LIST" ]; then
    variants="$(printf '%s\n' "$VARIANT_LIST" | tr ',' '\n' | sed 's/^ *//; s/ *$//')"
  else
    variants="$(build_variant_list)"
  fi

  # Determine parallelism
  if [ -z "$PARALLEL_JOBS" ]; then
    if command -v nproc &>/dev/null; then
      PARALLEL_JOBS="$(nproc)"
    elif [ -f /proc/cpuinfo ]; then
      PARALLEL_JOBS="$(grep -c ^processor /proc/cpuinfo)"
    else
      PARALLEL_JOBS=2
    fi
    # Cap at 4 to avoid excessive memory usage
    if [ "$PARALLEL_JOBS" -gt 4 ]; then
      PARALLEL_JOBS=4
    fi
  fi

  echo "::group::Building variants"

  # fix later: xargs parallel execution causes silent failures (exit 123)
  # with exported functions and associative arrays. Fall back to sequential.
  # if [ "$PARALLEL_JOBS" -gt 1 ] && command -v xargs &>/dev/null; then
  #   printf '%s\n' "$variants" | grep -v '^$' | \
  #     xargs -I{} -P "$PARALLEL_JOBS" bash -c 'build_one "$@"' _ "{}" "$TARGET" "$PLATFORM" "$ARCH" "$OUT_DIR"
  # else
    while IFS= read -r variant; do
      [ -z "$variant" ] && continue
      build_one "$variant" "$TARGET" "$PLATFORM" "$ARCH" "$OUT_DIR"
    done <<<"$variants"
  # fi
  echo "::endgroup::"
}

main
