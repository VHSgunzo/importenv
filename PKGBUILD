# Maintainer: VHSgunzo <vhsgunzo.github.io>
pkgname='importenv'
pkgver='0.0.6'
pkgrel='1'
pkgdesc='Launching an executable file with environment variables from a specific process id'
arch=("aarch64" "x86_64")
url="https://github.com/VHSgunzo/${pkgname}"
provides=("${pkgname}")
conflicts=("${pkgname}")
source=("https://github.com/VHSgunzo/${pkgname}/releases/download/v${pkgver}/${pkgname}-${CARCH}-Linux")
sha256sums=('SKIP')

package() {
    install -Dm755 "${pkgname}-${CARCH}-Linux" "$pkgdir/usr/bin/${pkgname}"
}