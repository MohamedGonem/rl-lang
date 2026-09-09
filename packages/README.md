# Packages

Packaging templates for distributing rl-lang across platforms. These are starting points for dedicated package repositories.

## Platforms

| Directory | Format | Maintainer target |
|-----------|--------|-------------------|
| `debian/` | `.deb` | Debian/Ubuntu PPA |
| `fedora/` | `.rpm` | Fedora COPR |
| `arch/` | `PKGBUILD` | AUR |
| `gentoo/` | `.ebuild` | Overlay repo |
| `nix/` | `.nix` | Nixpkgs or flake |
| `macos/` | Homebrew formula | Homebrew tap |
| `windows/` | Chocolatey `.nuspec` | Chocolatey community repo |
| `winget/` | WinGet manifest (YAML) | winget-pkgs repo |
| `snap/` | `snapcraft.yaml` | Snap Store |
| `flatpak/` | Flatpak manifest (YAML) | Flathub |

## How to use

Each directory contains a template that you adapt for your own package repo. See [PUBLISHING.md](PUBLISHING.md) for per-platform submission steps.

### Build from source (all platforms)

```bash
cargo build --release --all-features
```

The release binaries are: `rl`, `rlc`, `rld`. Man page: `man/rl.1`, info page: `man/rl.info`.

### Version bumps

Update the version in `Cargo.toml` and in each packaging file before publishing.

### Release artifacts

The GitHub release workflow produces per-platform `.tar.gz` (Linux/macOS) and `.zip` (Windows) archives. Package templates can fetch from these or build from source.
