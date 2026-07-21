load("@rules_rs//rs:rust_binary.bzl", "rust_binary")
load("@qtbridge//third_party:rules.bzl", "package_win32_qt")

rust_binary(
  name = "hello_world_test",
  deps = [
    "@qtbridge//crates/qtbridge",
    "//hello_world_test/translations:translations_rs",
    "//hello_world_test/CustomColor:CustomColor_rs",
    "//hello_world_test/qml:qml_rs",
  ],
  srcs = glob(["src/**/*.rs"]),
  compile_data = [],
  visibility = ["//visibility:public"],
)

package_win32_qt(
  name = "hello_world_test_tar",
  app = ":hello_world_test",
)
