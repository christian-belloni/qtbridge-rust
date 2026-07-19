load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@rules_rs//rs:rust_static_library.bzl", "rust_static_library")
load("@crates//:defs.bzl", "all_crate_deps")

rust_library(
  name = "qtbridge",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  visibility = ["//visibility:public"],
  compile_data = ["README.md"],
)

rust_static_library(
  name = "qtbridge_static",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  visibility = ["//visibility:public"],
  compile_data = ["README.md"],
)
