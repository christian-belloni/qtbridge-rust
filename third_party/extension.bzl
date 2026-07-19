load(":repository.bzl", "download_package")

def _download_msys2_packages_impl(_):
  download_package(
    name = "qtwin32-x86",
    arch = "x86",
  )
  download_package(
    name = "qtwin32-arm",
    arch = "arm",
  )


msys2qt = module_extension(
  implementation = _download_msys2_packages_impl
)
