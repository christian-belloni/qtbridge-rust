load("@cxx.rs//tools/bazel:rust_cxx_bridge.bzl", "rust_cxx_bridge")
load("@rules_cc//cc:defs.bzl", "cc_library")

def rust_bridge(name):
  rust_proxy_src = "src/%s/proxy_rust_bridge.rs" % name
  cpp_proxy_src = "src/%s/proxy_cpp_bridge.rs" % name
  rust_bridge = "%s_rust_bridge" % name
  cpp_bridge = "%s_cpp_bridge" % name
  
  rust_cxx_bridge(
    name = rust_bridge,
    src = rust_proxy_src,
  )
  
  rust_cxx_bridge(
    name = cpp_bridge,
    src = cpp_proxy_src,
  )
  
  cc_library(
    name = name,
    srcs = [
      "%s/source" % rust_bridge,
      "%s/source" % cpp_bridge,
    ],
    hdrs = [
      "%s/header" % rust_bridge,
      "%s/header" % cpp_bridge,
    ],
    deps = ["shared-include"],
    include_prefix = "qtbridge-interfaces",
  )
