config_setting(
  name = "windows_x86",
  constraint_values = [
    "@platforms//os:windows",
    "@platforms//cpu:x86_64",
  ]
)

config_setting(
  name = "windows_arm",
  constraint_values = [
    "@platforms//os:windows",
    "@platforms//cpu:aarch64",
  ]
)

alias(
  name = "qt",
  actual = select({
    "@platforms//os:osx": "@qtosx-arm//:qt",
    ":windows_x86": "@qtwin32-x86//:qt",
    ":windows_arm": "@qtwin32-arm//:qt",
  }),
  visibility = ["//visibility:public"]
)

alias(
  name = "qt_shared",
  actual = select({
    "@platforms//os:osx": "@qtosx-arm//:shared_libraries",
    ":windows_x86": "@qtwin32-x86//:shared_libraries",
    ":windows_arm": "@qtwin32-arm//:shared_libraries",
  }),

  visibility = ["//visibility:public"]
)

alias(
  name = "plugins",
  actual = select({
    "@platforms//os:osx": "@qtosx-arm//:plugins",
    ":windows_x86": "@qtwin32-x86//:plugins",
    ":windows_arm": "@qtwin32-arm//:plugins",
  }),
  visibility = ["//visibility:public"]
)

alias(
  name = "qml",
  actual = select({
    "@platforms//os:osx": "@qtosx-arm//:qml",
    ":windows_x86": "@qtwin32-x86//:qml",
    ":windows_arm": "@qtwin32-arm//:qml",
  }),
  visibility = ["//visibility:public"]
)

alias(
  name = "rcc",
  actual = select({
    "@platforms//os:osx": "@qtosx-arm//:rcc",
    ":windows_x86": "@qtwin32-x86//:rcc",
    ":windows_arm": "@qtwin32-arm//:rcc",
  }),
  visibility = ["//visibility:public"]
)

alias(
  name = "lconvert",
  actual = select({
    "@platforms//os:osx": "@qtosx-arm//:lconvert",
    ":windows_x86": "@qtwin32-x86//:lconvert",
    ":windows_arm": "@qtwin32-arm//:lconvert",
  }),
  visibility = ["//visibility:public"]
)
