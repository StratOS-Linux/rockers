# Maintainer: Magitian <magitian@duck.com>
pkgname='rockers'
pkgver=0.3
pkgrel=4
pkgdesc="StratOS's package manager manager"
arch=('x86_64')
url='https://github.com/stratos-linux/rockers'
license=('GPL-3.0-or-later')
makedepends=('rust' 'git')
depends=(
	'paru-bin'
	'flatpak'
)
provides=('rock')
conflicts=('rock')
source=()
noextract=()

build() {
	cargo build --release
}

package() {
	install -Dm755 "$startdir/target/release/rock" -t "$pkgdir/usr/bin/"
	install -Dm644 "$startdir/rock.fish" "$pkgdir/usr/share/fish/completions/rock.fish"
}

