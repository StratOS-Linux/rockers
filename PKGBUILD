# Maintainer: Magitian <magitian@duck.com>
pkgname='rockers'
pkgver='0.2'
pkgrel=3
pkgdesc="StratOS's package manager manager"
arch=('x86_64')
url='https://github.com/stratos-linux/rockers'
license=('GPL-3.0-or-later')
makedepends=(
	'rust'
	'git'
)

optdepends=(
	'yay-bin: AUR support'
	'flatpak: Flatpak support'
)
provides=('rock')
conflicts=('rock')
source=()
prepare() {
  cd $srcdir
  cargo build -r --target-dir=$srcdir/
}

package() {
	install -Dm755 "$srcdir/release/rock" -vt "$pkgdir"/usr/bin
	mkdir -p "$pkgdir/usr/share/fish/completions/"
	install -Dm644 "$startdir/rock.fish" -vt "$pkgdir"/usr/share/fish/completions/
}
