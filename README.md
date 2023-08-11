# rulesjs_cc_rust_extension_example
A bazel-built example of JS calling into rust, calling into v8 APIs that are not exposed in napi.

This depends on https://github.com/bazelbuild/rules_nodejs/pull/3679 to expose the node headers from v8 as a `cc_library`.

Once those headers are exposed, that `cc_library` defined by `native.cc` can depend on the headers (from the target runtime node toolchain).

The API in native.cc could be exposed directly to JS code, but instead we have a rust library that links against it and can implement additional logic in safe rust, while calling into v8 as needed.
The rust library is compiled into a `.so` that can be loaded by node at runtime.
