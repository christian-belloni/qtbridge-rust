load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@crates//:defs.bzl", "all_crate_deps")

rust_library(
  name = "qtbridge-build-utils",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  visibility = ["//crates:__subpackages__"],
)
