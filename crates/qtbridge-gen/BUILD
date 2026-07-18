load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@rules_rs//rs:rust_proc_macro.bzl", "rust_proc_macro")
load("@crates//:defs.bzl", "all_crate_deps")

rust_proc_macro(
  name = "qtbridge-gen",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  visibility = ["//crates:__subpackages__"],
)
