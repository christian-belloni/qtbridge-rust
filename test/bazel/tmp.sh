get_package_url () {
  curl -s https://packages.msys2.org/packages/$1| lynx -dump -nonumbers -listonly -stdin | grep tar.zst | grep aarch64
}


# qtbase deps
get_package_url mingw-w64-clang-aarch64-dbus
get_package_url mingw-w64-clang-aarch64-double-conversion
  get_package_url mingw-w64-clang-aarch64-libc++

get_package_url mingw-w64-clang-aarch64-freetype
  get_package_url mingw-w64-clang-aarch64-brotli
  get_package_url mingw-w64-clang-aarch64-bzip2
  get_package_url mingw-w64-clang-aarch64-cc-libs
  get_package_url mingw-w64-clang-aarch64-harfbuzz
  get_package_url mingw-w64-clang-aarch64-libpng
  get_package_url mingw-w64-clang-aarch64-zlib

get_package_url mingw-w64-clang-aarch64-glib2
  get_package_url mingw-w64-clang-aarch64-gettext-runtime
  get_package_url mingw-w64-clang-aarch64-libffi
  get_package_url mingw-w64-clang-aarch64-pcre2
  # get_package_url mingw-w64-clang-aarch64-python
  # get_package_url mingw-w64-clang-aarch64-python-packaging
  get_package_url mingw-w64-clang-aarch64-zlib

get_package_url mingw-w64-clang-aarch64-harfbuzz
  get_package_url mingw-w64-clang-aarch64-graphite2
get_package_url mingw-w64-clang-aarch64-icu
get_package_url mingw-w64-clang-aarch64-libb2
get_package_url mingw-w64-clang-aarch64-libjpeg-turbo
get_package_url mingw-w64-clang-aarch64-libpng
get_package_url mingw-w64-clang-aarch64-md4c
get_package_url mingw-w64-clang-aarch64-openssl
get_package_url mingw-w64-clang-aarch64-pcre2

  get_package_url mingw-w64-clang-aarch64-bzip2
  get_package_url mingw-w64-clang-aarch64-wineditline
  get_package_url mingw-w64-clang-aarch64-zlib

get_package_url mingw-w64-clang-aarch64-sqlite3
  get_package_url mingw-w64-clang-aarch64-tcl
get_package_url mingw-w64-clang-aarch64-vulkan-headers
get_package_url mingw-w64-clang-aarch64-vulkan-loader
get_package_url mingw-w64-clang-aarch64-zlib
get_package_url mingw-w64-clang-aarch64-zstd

#qtdeclarative deps
get_package_url mingw-w64-clang-aarch64-qt6-shadertools
get_package_url mingw-w64-clang-aarch64-qt6-svg
