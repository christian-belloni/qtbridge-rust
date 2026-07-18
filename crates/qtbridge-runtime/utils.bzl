load("@cxx.rs//tools/bazel:rust_cxx_bridge.bzl", "rust_cxx_bridge")
load("@rules_cc//cc:defs.bzl", "cc_library")

def rust_bridge(name):
  rust_source = "src/%s.rs" % name
  cpp_file = "src/cpp/%s.cpp" % name
  h_file = "src/cpp/%s.h" % name
  
  rust_cxx_bridge(
    name = "_%s-bridge" % name,
    src = rust_source
  )
  
  cc_library(
    name = name,
    srcs = native.glob([cpp_file], allow_empty = True) + ["_%s-bridge/generated" % name, h_file],
    deps = ["shared-include"],
    includes = ["."],
    # strip_include_prefix = "src",
  )
  

