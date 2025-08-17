# Maintainer: Magitian <magitian@duck.com>
pkgname='rockers'
pkgver=0.2
pkgrel=4
pkgdesc="StratOS's package manager manager"
arch=('x86_64')
url='https://github.com/stratos-linux/rockers'
license=('GPL-3.0-or-later')
makedepends=('rust' 'git')
optdepends=(
	'yay-bin: AUR support'
	'flatpak: Flatpak support'
)
provides=('rock')
conflicts=('rock')
source=()
noextract=()

build() {
	cargo build --release --locked --target-dir=/tmp/rockers.pacman
}

package() {
	install -Dm755 "/tmp/rockers.pacman/release/rock" -t "$pkgdir/usr/bin/"
	install -Dm644 "$startdir/rock.fish" "$pkgdir/usr/share/fish/completions/rock.fish"
}

