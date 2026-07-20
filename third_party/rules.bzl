load("@bazel_skylib//rules/directory:providers.bzl", "DirectoryInfo")
load("@rules_cc//cc:defs.bzl", "cc_library")
load("@rules_rs//rs:rust_library.bzl", "rust_library")

_SCRIPT = """
BUILD_DIR=$(pwd)/.build_dir
OUT=$(pwd)/{out}
APP={app}
PLUGINS={plugins}
QML={qml}
LIBS="{libs}"

mkdir $BUILD_DIR

cp $APP $BUILD_DIR/

cp -r $PLUGINS/* $BUILD_DIR/

cp -r $QML $BUILD_DIR/

for lib in $LIBS;
do
  cp $lib $BUILD_DIR
done

cd $BUILD_DIR
echo '''
[Paths]
Prefix = .
Imports = qml
''' > qt.conf
tar -hcvz -f $OUT *
"""

def _package_win32_qt(ctx):
  app = ctx.file.app
  libs = ctx.files.shared_libraries
  plugins = ctx.attr.plugins[DirectoryInfo]
  qml = ctx.attr.qml[DirectoryInfo]
  tar = ctx.actions.declare_file("%s.tar.gz" % ctx.attr.name)
  ctx.actions.run_shell(
    command = _SCRIPT.format(
      out = tar.path,
      app = app.path,
      plugins = plugins.path,
      qml = qml.path,
      libs = " ".join([ lib.path for lib in libs ])
    ),
    inputs = depset(libs + [app], transitive = [plugins.transitive_files, qml.transitive_files]),
    outputs = [tar]
  )

  return DefaultInfo(files = depset([tar]))


package_win32_qt = rule(
  implementation = _package_win32_qt,
  attrs = {
    "app": attr.label(allow_single_file = True, mandatory = True),
    "shared_libraries": attr.label(allow_files = True, default = Label("//:qt_shared")),
    "plugins": attr.label(allow_files = True, default = Label("//:plugins")),
    "qml": attr.label(allow_files = True, default = Label("//:qml")),
  }
)

QtResource = provider(doc = "", fields = ["name", "binary", "qrc", "qmldir"])

def _qt_resource_impl(ctx):
  out_bin = ctx.actions.declare_file("%s.rcc" % ctx.attr.name)
  srcs = [ f for f in ctx.files.srcs if (not f.path.endswith("qrc") and not f.path.endswith("qmldir")) ]
  
  if ctx.attr.qmldir != None:
    qmldir = ctx.file.qmldir
  else:
    qmldir = [ f for f in ctx.files.srcs if f.path.endswith("qmldir") ][0]

  if ctx.attr.qrc != None:
    qrc = ctx.file.qrc
  else:
    qrc = [ f for f in ctx.files.srcs if f.path.endswith("qrc") ][0]

  ctx.actions.run(
    executable = ctx.executable._rcc,
    arguments = [qrc.path, "--binary", "-o", out_bin.path],
    inputs = srcs + [ qrc, qmldir ],
    outputs = [out_bin]
  )

  return [DefaultInfo(files = depset([out_bin])), QtResource(name = ctx.attr.name, binary = out_bin, qmldir = ctx.file.qmldir)]

  

_qt_resource = rule(
  implementation = _qt_resource_impl,
  attrs = {
    "qrc": attr.label(allow_single_file = True),
    "qmldir": attr.label(allow_single_file = True),
    "srcs": attr.label_list(allow_files = True),
    "_rcc": attr.label(allow_single_file = True, executable = True, cfg = "exec", default = Label("//:rcc")),
  }
)

def _qt_cpp_resource_impl(ctx):
  return DefaultInfo(files = ctx.attr.res[QtResource].cpp)

_cpp_resource = rule(
  implementation = _qt_cpp_resource_impl,
  attrs = {
    "res": attr.label(providers = [QtResource])
  }
)

def _qt_binary_resource_impl(ctx):
  return DefaultInfo(files = ctx.attr.res[QtResource].binary)

_binary_resource = rule(
  implementation = _qt_binary_resource_impl,
  attrs = {
    "res": attr.label(providers = [QtResource])
  }
)

def qt_resource(*, name, srcs, qrc = None, qmldir = None, **kwargs):
  _qt_resource(
    name = "%s" % name,
    qrc = qrc,
    srcs = srcs,
    qmldir = qmldir,
    **kwargs,
  )

  rust_library(
    name = "%s_rs" % name,
    crate_name = name,
    srcs = ["@qtbridge//third_party:lib.rs"],
    compile_data = [name],
    rustc_env = {
      "PREFIX": "/qt/qml/%s" %name,
      "RCC_FILE": "$(location %s)" % name
    },
    deps = ["@qtbridge//crates/qtbridge"]
  )

def _qt_resource_package_impl(ctx):
  pass


_qt_resource_package = rule(
  implementation = _qt_resource_package_impl
)

def qt_resource_package(*, name, deps, **kwargs):
  _qt_resource_package(
    name = name
  )

  for dep in deps:
    pass
  pass
