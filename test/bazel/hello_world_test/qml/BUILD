load("@qtbridge//third_party:rules.bzl", "qt_resource")

qt_resource(
  name = "qml",
  qrc = "res.qrc",
  srcs = ["Main.qml"],
  qmldir = "qmldir",
  visibility = ["//visibility:public"]
)
exports_files(["Main.qml"])
