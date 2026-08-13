#!/usr/bin/env bash
set -euo pipefail
# usage: scripts/bump-version.sh 1.2.0        (explicit version)
#        scripts/bump-version.sh patch|minor|major   (bump from current)

M="Cargo.toml"

cur=$(sed -n '/^\[workspace.package\]/,/^\[/s/^version = "\([^"]*\)"/\1/p' "$M" | head -n1)
if [[ -z "$cur" ]]; then
    echo "could not parse current version in [workspace.package]" >&2
    exit 1
fi

next="${1:?usage: bump-version.sh [patch|minor|major|<version>]}"
if [[ ! "$next" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    IFS=. read -r maj min pat <<< "$cur"
    case "$next" in
        patch) pat=$((pat + 1)) ;;
        minor) min=$((min + 1)); pat=0 ;;
        major) maj=$((maj + 1)); min=0; pat=0 ;;
        *) echo "bad version: $next" >&2; exit 1 ;;
    esac
    next="$maj.$min.$pat"
fi

# update [workspace.package] version
sed -i "/^\[workspace.package\]/,/^\[/s/^version = \"[^\"]*\"/version = \"$next\"/" "$M"

# update the rl-* entries under [workspace.dependencies]
sed -i -E '/^\[workspace.dependencies\]/,/^\[/{
    /^rl-[a-z-]+ = \{ path = "crates\/rl-/{
        s/, version = "[^"]*" *\}$/ }/
        s/ \}$/, version = "'"$next"'" }/
    }
}' "$M"

echo "rl-lang $cur -> $next"