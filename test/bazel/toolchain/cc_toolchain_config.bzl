load("@rules_cc//cc:defs.bzl", "cc_common", "CcToolchainConfigInfo")
load("@rules_cc//cc:action_names.bzl", "ACTION_NAMES")
load(
    "@rules_cc//cc:cc_toolchain_config_lib.bzl",
    "feature",
    "flag_group",
    "flag_set",
    "tool_path",
    "artifact_name_pattern"
)

all_link_actions = [ # NEW
    ACTION_NAMES.cpp_link_executable,
    ACTION_NAMES.cpp_link_dynamic_library,
    ACTION_NAMES.cpp_link_nodeps_dynamic_library,
]


def _impl(ctx):
    tool_paths = [
        tool_path(
            name = "gcc",
            path = "/opt/homebrew/bin/x86_64-w64-mingw32-g++",
        ),
        tool_path(
            name = "ld",
            path = "/opt/homebrew/bin/x86_64-w64-mingw32-ld",
        ),
        tool_path(
            name = "ar",
            path = "/opt/homebrew/bin/x86_64-w64-mingw32-ar",
        ),
        tool_path(
            name = "cpp",
            path = "/opt/homebrew/bin/x86_64-w64-mingw32-g++",
        ),
        tool_path(
            name = "gcov",
            path = "/bin/false",
        ),
        tool_path(
            name = "nm",
            path = "/bin/false",
        ),
        tool_path(
            name = "objdump",
            path = "/bin/false",
        ),
        tool_path(
            name = "strip",
            path = "/bin/false",
        ),
    ]
    
    
    
    features = [ # NEW
        feature(
            name = "default_linker_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = all_link_actions,
                    flag_groups = ([
                        flag_group(
                            flags = [
                            "-static-libgcc",
                            # "--unwindlib=libunwind",
                            "-pthread",
                            ],
                        ),
                    ]),
                ),
            ],
        ),
    ]
    return cc_common.create_cc_toolchain_config_info(
        ctx = ctx,
        features = features,
        cxx_builtin_include_directories = [
          "/opt/homebrew/Cellar/mingw-w64/14.0.0_1/toolchain-x86_64/x86_64-w64-mingw32/include",
          "/opt/homebrew/Cellar/mingw-w64/14.0.0_1/toolchain-x86_64/x86_64-w64-mingw32/include/c++/16.1.0",
          "/opt/homebrew/Cellar/mingw-w64/14.0.0_1/toolchain-x86_64/lib/gcc/x86_64-w64-mingw32/16.1.0/include"
        ],
        toolchain_identifier = "k8-toolchain",
        compiler = "gcc",
        tool_paths = tool_paths,
        artifact_name_patterns = [
          artifact_name_pattern(
              category_name = "executable",
              prefix = "",
              extension = ".exe",
          ),
          artifact_name_pattern(
              category_name = "dynamic_library",
              prefix = "lib",
              extension = ".dll",
          ),
          artifact_name_pattern(
              category_name = "static_library",
              prefix = "lib",
              extension = ".a",
          )
        ]
    )

cc_toolchain_config = rule(
    implementation = _impl,
    attrs = {},
    provides = [CcToolchainConfigInfo],
)



