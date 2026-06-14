#pragma once

// Function template. hicc binds each explicit instantiation separately
// via `#[cpp(func = "ret f<T>(args)")]`.

template <typename T>
T identity(T x) { return x; }

template <typename T>
T add_tmpl(T a, T b) { return a + b; }
