load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@rules_rs//rs:rust_static_library.bzl", "rust_static_library")
load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_cc//cc:defs.bzl", "cc_library", "cc_binary")
load(":utils.bzl", "rust_bridge", "path_to_name", "GENERATED_FILES_BRIDGE", "GENERATED_FILES_CPP")

rust_library(
  name = "qtbridge-type-lib",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps() + [":qtbridge-type-lib-cc"],
  visibility = ["//visibility:public"],
  alwayslink = True
)

rust_static_library(
  name = "qtbridge-type-lib_static",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  link_deps = [":qtbridge-type-lib-cc"],
  visibility = ["//visibility:public"],
)

[ rust_bridge(bridge_file) for bridge_file in GENERATED_FILES_BRIDGE ]


cc_library(
  name = "generated_includes",
  hdrs = glob(["src/generated/**/*.h"]),
  include_prefix = "qtbridge-type-lib",
  linkstatic = True,
  alwayslink = True,
)

cc_library(
  name = "shared-include",
  hdrs = ["src/rustconv.h"],
  deps = ["//:qt", "@cxx.rs//:core", ":generated_includes"],
  includes = ["src"]
)

cc_library(
  name = "qtbridge-type-lib-cc",
  srcs = GENERATED_FILES_CPP,
  deps = [ path_to_name(bridge_file) for bridge_file in GENERATED_FILES_BRIDGE ] + ["shared-include"],
  visibility = ["//visibility:public"],
)

cc_library(
  name = "qtbridge-type-lib-bin",
  srcs = GENERATED_FILES_CPP,
  deps = [ path_to_name(bridge_file) for bridge_file in GENERATED_FILES_BRIDGE ] + ["shared-include"],
  visibility = ["//crates:__subpackages__"],
  alwayslink = True,
)
