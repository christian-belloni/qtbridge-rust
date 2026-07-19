load("@rules_rs//rs:rust_binary.bzl", "rust_binary")
load("@qtbridge//third_party:rules.bzl", "package_win32_qt")

rust_binary(
  name = "hello_world_test",
  deps = [
    "@qtbridge//crates/qtbridge"
  ],
  srcs = glob(["src/**/*.rs"]),
  compile_data = glob(["qml/**/*.qml"]),
  visibility = ["//visibility:public"],
  # data = ["@qtbridge//:qt_shared"]
)

package_win32_qt(
  name = "hello_world_test_tar",
  app = ":hello_world_test",
)
