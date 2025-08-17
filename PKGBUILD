# Maintainer: Magitian <magitian@duck.com>
pkgname='rockers'
pkgver='0.2'
pkgrel=2
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
# prepare() {
#    cp -r "$startdir/**.**" "$srcdir/"
# }
build() {
  cd $srcdir
	cargo build -r
}

package() {
	install -Dm755 "$srcdir/$pkgname/target/release/rock" -vt "$pkgdir"/usr/bin
	mkdir -p "$pkgdir/usr/share/fish/completions/"
	install -Dm644 "$srcdir/$pkgname/rock.fish" -vt "$pkgdir"/usr/share/fish/completions/
}
