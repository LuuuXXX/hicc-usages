#include "custom_deleter.h"

namespace custom_deleter_ns {

IntArrayPtr make_int_array(size_t n) {
    int* p = new int[n];
    for (size_t i = 0; i < n; ++i) p[i] = static_cast<int>(i * i);
    return IntArrayPtr(p);
}

int read_at(const IntArrayPtr& arr, size_t i) {
    return arr[i];
}

size_t bytes_allocated(size_t n) {
    return n * sizeof(int);
}

int custom_deleter_status() {
    return 1;
}

IntArrayHandle* make_int_array_handle(size_t n) {
    return new IntArrayHandle{make_int_array(n), n};
}

int handle_read_at(const IntArrayHandle* h, size_t i) {
    return h->ptr[i];
}

void destroy_int_array_handle(IntArrayHandle* h) {
    delete h;
}

size_t handle_size(const IntArrayHandle* h) {
    return h->size;
}

int custom_deleter_anchor() { return 31; }
}
