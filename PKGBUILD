# Maintainer: the lonesome programmer <m@thelonesomeprogrammer.dk>
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=rusty-bar-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple icon loving bar for wayland"
url="https://github.com/thelonesomeprogrammer/rusty-bar/"
license=("GNU GPLv3")
arch=("x86_64")
provides=("rusty-bar")
conflicts=("rusty-bar")
source=("https://github.com/thelonesomeprogrammer/rusty-bar/releases/download/v$pkgver/rusty-bar-$pkgver-x86_64.tar.gz")
sha256sums=("5d1f148e5b4175b0d47c831efa311b0f3da438a1f81ea6042a0139b2be509ce2")

package() {
    install -Dm755 rusty-bar -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
