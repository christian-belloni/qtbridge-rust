load("@rules_rs//rs:rust_binary.bzl", "rust_binary")
load("@qtbridge//third_party:rules.bzl", "package_win32_qt", "qt_resource_package")

rust_binary(
  name = "hello_world_test",
  deps = [
    "@qtbridge//crates/qtbridge",
  ],
  srcs = glob(["src/**/*.rs"]),
  compile_data = ["//hello_world_test/CustomColor:CustomColor", "//hello_world_test/qml:qml" ],
  visibility = ["//visibility:public"],
)

qt_resource_package(
  name = "resources",
  deps = ["//hello_world_test/CustomColor", "//hello_world_test/qml" ],
)

package_win32_qt(
  name = "hello_world_test_tar",
  app = ":hello_world_test",
)
