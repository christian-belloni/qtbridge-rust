load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_cc//cc:defs.bzl", "cc_library")
load(":utils.bzl", "rust_bridge")

rust_library(
  name = "qtbridge-interfaces",
  srcs = glob(["src/**/*.rs"]),
  deps = all_crate_deps(),
  visibility = ["//visibility:public"],
  link_deps = ["qtbridge-interfaces-cc"]
)

FILES_BRIDGE = [
  "qabstract_item_model",
  "qlist_model",
  "qobject",
  "qparser_status",
  "qtable_model",
]

[ rust_bridge(name) for name in FILES_BRIDGE ]

cc_library(
  name = "shared-include",
  hdrs = glob(["src/**/*.h"]),
  deps = [
    "//:qt", 
    ":generated_includes", 
    "//crates/qtbridge-type-lib:qtbridge-type-lib-cc",
    "//crates/qtbridge-runtime:qtbridge-runtime-cc",
  ],
  includes = ["src"]
)

cc_library(
  name = "generated_includes",
  hdrs = glob(["src/**/*.h"]),
  include_prefix = "qtbridge-interfaces",
)

cc_library(
  name = "qtbridge-interfaces-cc",
  srcs = [
    "src/qabstract_item_model/cpp/QAbstractItemModelProxyCpp.cpp",
    "src/qlist_model/cpp/QListModelProxyCpp.cpp",
    "src/qobject/cpp/QObjectProxyCpp.cpp",
    "src/qparser_status/cpp/QParserStatusProxyCpp.cpp",
    "src/qtable_model/cpp/QTableModelProxyCpp.cpp",
  ],
  hdrs = [
    "src/qabstract_item_model/cpp/QAbstractItemModelProxyCpp.h",
    "src/qlist_model/cpp/QListModelProxyCpp.h",
    "src/qobject/cpp/QObjectProxyCpp.h",
    "src/qparser_status/cpp/QParserStatusProxyCpp.h",
    "src/qtable_model/cpp/QTableModelProxyCpp.h",
  ],
  deps = FILES_BRIDGE + [
    "@cxx.rs//:core", 
  ],
)
