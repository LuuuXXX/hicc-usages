#pragma once

// Static members + methods. Static methods are bound in `import_lib!` with the
// fully-qualified C++ name (Class::method), so they look like free functions
// to Rust.

class Registry {
public:
    Registry()  { ++live_count_; }
    ~Registry() { --live_count_; }

    static int live_count() { return live_count_; }
    static int next_id()    { return next_id_++; }

private:
    static int live_count_;
    static int next_id_;
};
