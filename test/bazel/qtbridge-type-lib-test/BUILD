load("@rules_rs//rs:rust_binary.bzl", "rust_binary")

rust_binary(
  name = "qtbridge-type-lib-test",
  deps = ["@qtbridge//crates/qtbridge-type-lib"],
  srcs = glob(["src/**/*.rs"]),
  visibility = ["//visibility:public"],
)
