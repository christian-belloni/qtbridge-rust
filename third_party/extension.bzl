load(":repository.bzl", "download_package")

osx_tag = tag_class(
  attrs = {
   "url": attr.string()
  }
)

def _download_qt_impl(mctx):
  download_package(
    name = "qtwin32-x86",
    os = "windows",
    arch = "x86",
    tpl = "//third_party:qtwin32-x86.BUILD.bazel",
  )
  download_package(
    name = "qtwin32-arm",
    os = "windows",
    arch = "arm",
    tpl = "//third_party:qtwin32-arm.BUILD.bazel",
  )

  for mod in mctx.modules:
    for tag in mod.tags.osx:
      download_package(
        name = "qtosx-arm",
        os = "osx",
        arch = "arm",
        tpl = "//third_party:qtosx-arm.BUILD.bazel",
        url = tag.url
      )
      break


qt = module_extension(
  implementation = _download_qt_impl,
  tag_classes = {
    "osx": osx_tag
  }
)
