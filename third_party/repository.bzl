def _download_package(rctx):
  if rctx.attr.arch == "x86":
    _download_x86(rctx)
  elif rctx.attr.arch == "arm":
    _download_arm(rctx)


_msys_urls = [
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-qt6-base-6.11.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-qt6-declarative-6.11.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-brotli-1.2.0-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-bzip2-1.0.8-3-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-dbus-1.16.2-3-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-double-conversion-3.4.0-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-freetype-2.14.3-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-gettext-runtime-1.0-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-glib2-2.88.2-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-graphite2-1.3.15-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-harfbuzz-14.2.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-icu-78.3-3-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-libb2-0.98.1-3-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-libc%2B%2B-22.1.8-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-libffi-3.7.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-libjpeg-turbo-3.2.0-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-libpng-1.6.58-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-md4c-0.5.3-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-openssl-3.6.3-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-pcre2-10.47-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-qt6-shadertools-6.11.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-qt6-svg-6.11.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-sqlite3-3.53.3-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-tcl-8.6.18-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-vulkan-headers-1~1.4.350.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-vulkan-loader-1~1.4.350.1-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-wineditline-2.208-1-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-zlib-1.3.2-2-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-zstd-1.5.7-2-any.pkg.tar.zst",
  "https://mirror.msys2.org/mingw/{prefix}/mingw-w64-{arch}-libiconv-1.19-1-any.pkg.tar.zst",
]

def _download_arm(rctx):
  for url in _msys_urls:
    rctx.download_and_extract(
      url = url.format(arch = "clang-aarch64", prefix = "clangarm64"),
      strip_prefix = "clangarm64"
    )

  rctx.template("BUILD.bazel", rctx.attr._tpl_arm)

def _download_x86(rctx):
  for url in _msys_urls:
    rctx.download_and_extract(
      url = url.format(arch = "clang-x86_64", prefix = "clang64"),
      strip_prefix = "clang64"
    )
  # rctx.download_and_extract(
  #   url = "https://download.qt.io/online/qtsdkrepository/windows_x86/desktop/qt6_6111/qt6_6111_llvm_mingw/qt.qt6.6111.win64_llvm_mingw/6.11.1-0-202605090529qtbase-Windows-Windows_11_24H2-Clang-Windows-Windows_11_24H2-X86_64.7z",

  # )
  # rctx.download_and_extract(
  #   url = "https://download.qt.io/online/qtsdkrepository/windows_x86/desktop/qt6_6111/qt6_6111_llvm_mingw/qt.qt6.6111.win64_llvm_mingw/6.11.1-0-202605090529qtdeclarative-Windows-Windows_11_24H2-Clang-Windows-Windows_11_24H2-X86_64.7z",

  # )

  # rctx.download_and_extract(
  #   url = "https://download.qt.io/online/qtsdkrepository/windows_x86/desktop/qt6_6111/qt6_6111_llvm_mingw/qt.qt6.6111.win64_llvm_mingw/6.11.1-0-202605090529llvm-mingw-20231128-ucrt-x86_64-runtime.7z",
  #   output = "bin"
  # )

  rctx.template("BUILD.bazel", rctx.attr._tpl_x86)

download_package = repository_rule(
  implementation = _download_package,
  attrs = {
    "arch": attr.string(values = ["arm", "x86"]),
    "_tpl_x86": attr.label(allow_single_file = True, default = Label("qtwin32-x86.BUILD.bazel")),
    "_tpl_arm": attr.label(allow_single_file = True, default = Label("qtwin32-arm.BUILD.bazel")),
  }
)
