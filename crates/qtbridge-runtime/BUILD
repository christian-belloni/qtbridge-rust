load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@rules_rs//rs:rust_static_library.bzl", "rust_static_library")
load("@crates//:defs.bzl", "all_crate_deps")
load(":utils.bzl", "rust_bridge")
load("@rules_cc//cc:defs.bzl", "cc_library")

rust_library(
  name = "qtbridge-runtime",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps() + [":qtbridge-runtime-cc"],
  visibility = ["//visibility:public"],
  alwayslink = True
)

rust_static_library(
  name = "qtbridge-runtime_static",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  visibility = ["//crates:__subpackages__"],
  link_deps = [":qtbridge-runtime-cc"],
)

BRIDGE_FILES = [
  "dynamicmetaobjectbuilder",
  "dynamicmetaobjectdata",
  "qmetatypeget",
  "qresource",
  "rustobjectgetter",
  "qml_method_invoker",
]

[ rust_bridge(name) for name in BRIDGE_FILES ]


cc_library(
  name = "shared-include",
  hdrs = glob(["src/**/*.h"]),
  deps = ["//:qt", "@cxx.rs//:core", ":generated_includes", "//crates/qtbridge-type-lib:qtbridge-type-lib-cc"],
  includes = ["src"],
)

cc_library(
  name = "generated_includes",
  hdrs = glob(["src/**/*.h"]),
  include_prefix = "qtbridge-runtime",
)

cc_library(
  name = "qtbridge-runtime-cc",
  deps = [name for name in BRIDGE_FILES],
  visibility = ["//visibility:public"],
)
