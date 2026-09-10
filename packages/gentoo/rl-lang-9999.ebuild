# Copyright 2026 rl-lang contributors
# Distributed under the terms of the MIT license

EAPI=8

CRATES=""

inherit cargo

DESCRIPTION="Programming language with first-class VM and C transpiler"
HOMEPAGE="https://github.com/rl-lang/rl-lang"
SRC_URI="https://github.com/rl-lang/rl-lang/archive/v${PV}.tar.gz -> ${P}.tar.gz"

LICENSE="MIT Apache-2.0"
SLOT="0"
KEYWORDS="~amd64 ~arm64"
IUSE=""

BDEPEND="virtual/rust"
RDEPEND=""

src_install() {
  cargo_src_install

  # Remove default Cargo-installed binaries, reinstall manually for control
  rm -f "${ED}/usr/bin/rl" "${ED}/usr/bin/rlc" "${ED}/usr/bin/rld" 2>/dev/null || true

  install -Dm755 target/release/rl "${ED}/usr/bin/rl"
  install -Dm755 target/release/rlc "${ED}/usr/bin/rlc"
  dobin target/release/rld 2>/dev/null || true

  doman man/rl.1
  einstalldocs

  if [ -f man/rl.info ]; then
    install -Dm644 man/rl.info "${ED}/usr/share/info/rl.info"
  fi
}

pkg_postinst() {
  elog "Shell completions can be generated with: rl completions <shell>"
}
