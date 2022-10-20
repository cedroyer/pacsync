# Maintainer: Cédric ROYER <ced d°t royer à gmail d0t com>
pkgname=pacsync
pkgver=v2.0
pkgrel=1
pkgdesc=""
arch=('any')
url="https://github.com/cedroyer/pacsync"
license=('GPL2')
groups=()
depends=('sudo')
makedepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("https://github.com/cedroyer/pacsync/archive/refs/tags/${pkgver}.tar.gz")
noextract=()
sha256sums=('47e526129ee2ceb561bb70c9d8bdf83dfee7a54ffa2fa2b3153495dd2efa074a')

package() {
  cd "$pkgname-$pkgver"

  install -Dm644 COPYING "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  make DESTDIR="$pkgdir/" install
}
