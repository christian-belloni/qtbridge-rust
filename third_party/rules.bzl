load("@bazel_skylib//rules/directory:providers.bzl", "DirectoryInfo")

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
    "shared_libraries": attr.label(allow_files = True, mandatory = True),
    "plugins": attr.label(allow_files = True),
    "qml": attr.label(allow_files = True),
  }
)
