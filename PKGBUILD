# Maintainer: the lonesome programmer <m@thelonesomeprogrammer.dk>
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=rusty-bar-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple CNX based bar for X11"
url="https://thelonesomeprogrammer.github.io/rusty-bar/"
license=("MIT")
arch=("x86_64")
provides=("rusty-bar")
conflicts=("rusty-bar")
source=("https://github.com/thelonesomeprogrammer/rusty-bar/releases/download/v$pkgver/rusty-bar-$pkgver-x86_64.tar.gz")
sha256sums=("1a4cbac073452725073f474fe62259fc4f000212f0f4d715128704c6a643d67a")

package() {
    install -Dm755 rusty-bar -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
