#pragma once

// b has a default value of 10 in C++. Rust can't express defaults, so we expose
// the full signature and let callers pass the default explicitly.
int add(int a, int b = 10);
