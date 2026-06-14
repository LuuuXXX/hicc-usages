#pragma once

// Nested namespaces: hicc's #[cpp(func = "...")] just sees fully-qualified
// signatures, so namespaces are transparent. We expose top-level free wrappers.

namespace outer::inner::core {
    int add(int a, int b);
    int mul(int a, int b);
}

namespace outer::inner::util {
    int combined(int a, int b, int c);
}

// Top-level wrappers — keeps FFI surface simple.
int ns_add(int a, int b);
int ns_mul(int a, int b);
int ns_combined(int a, int b, int c);
