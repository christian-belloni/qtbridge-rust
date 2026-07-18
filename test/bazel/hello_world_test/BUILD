load("@rules_rs//rs:rust_binary.bzl", "rust_binary")

rust_binary(
  name = "hello_world_test",
  deps = ["@qtbridge//crates/qtbridge"],
  srcs = glob(["src/**/*.rs"]),
  compile_data = glob(["qml/**/*.qml"]),
  visibility = ["//visibility:public"],
)
