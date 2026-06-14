#include <stdio.h>
#include <inttypes.h>
#include "hicc_demo.h"

/*
 * Ownership rules for AbiClass objects:
 *
 *   Rust returns an AbiClass via factory functions (new_*).  The caller (C)
 *   receives ownership.  Every AbiClass is heap-allocated (Box<T>).
 *   When no longer needed, the object MUST be released by calling:
 *
 *       obj.methods->hicc_destroy(obj);
 *
 *   This corresponds to Rust's destructor (Drop).  The inner Box is freed.
 *
 *   ┌──────────────────────────────────────────────────────────────────┐
 *   │  Passing an AbiClass by value TO a Rust function TRANSFERS      │
 *   │  ownership — Rust's Drop fires and destroy is called           │
 *   │  automatically.  Calling destroy again in C after that would    │
 *   │  be a DOUBLE-FREE.  Never do:                                   │
 *   │                                                                  │
 *   │     obj = fn->new_X(...);                                        │
 *   │     fn->some_func(obj);     // ← ownership transferred           │
 *   │     obj.methods->hicc_destroy(obj); // ← BAD: double-free             │
 *   │                                                                  │
 *   │  Instead use the methods vtable (which takes &/pointer), keep   │
 *   │  ownership in C, then call destroy explicitly.                   │
 *   └──────────────────────────────────────────────────────────────────┘
 */

int main(void) {
    const struct Hicc_demo *fn = demo();

    /* ==================================================================
     * 1. Plain i32 — no allocation, no destroy needed
     * ================================================================== */
    int32_t sum = fn->add(3, 4);
    printf("add(3, 4) = %" PRId32 "\n", sum);

    int32_t neg = fn->negate(5);
    printf("negate(5) = %" PRId32 "\n", neg);

    /* ==================================================================
     * 2. Container<i32> — owned heap value, explicit destroy
     *
     *    Use the methods vtable (get takes &Container) so C keeps
     *    ownership, then destroy explicitly.
     * ================================================================== */
    {
        struct AbiClass_Container_i32 c = fn->new_container(42);
        int32_t val = *c.methods->get(&c);
        printf("container_value(42) = %" PRId32 "\n", val);
        c.methods->hicc_destroy(c);
    }

    /* ==================================================================
     * 3. Option<i32> — owned heap value, explicit destroy
     *
     *    as_ref(&opt) returns a pointer to the inner value without
     *    consuming the Option.  After reading, destroy explicitly.
     * ================================================================== */
    {
        struct AbiClass_Option_i32 opt = fn->new_option(99);
        const int32_t *p = opt.methods->as_ref(&opt);
        int64_t doubled = *p * 2LL;
        printf("double_option(Some(99)) = %" PRId64 "\n", doubled);
        opt.methods->hicc_destroy(opt);
    }

    /* ==================================================================
     * 4. Str — heap-allocated, explicit destroy
     * ================================================================== */
    {
        struct AbiClass_str s = fn->new_str();
        size_t len = s.methods->len(&s);
        printf("check_str(\"hello\") = %zu\n", len);
        s.methods->hicc_destroy(s);
    }

    /* ==================================================================
     * 5. Slice<Option<i32>> — pass by value transfers ownership to Rust
     * ================================================================== */
    {
        struct AbiClass_option_Option_i32 slice_val = fn->new_slice();
        size_t count = fn->count_some(slice_val);
        printf("count_some(&[Some(10), None, Some(30)]) = %zu\n", count);
    }

    /* ==================================================================
     * 6. Array<&'static str, 3> — pass by value transfers ownership to Rust
     * ================================================================== */
    {
        struct AbiClass_str_3 arr = fn->new_array();
        size_t total = fn->total_len(arr);
        printf("total_len([\"a\", \"bb\", \"ccc\"]) = %zu\n", total);
    }

    /* ==================================================================
     * 7. Point (POD struct) — no allocation, no destroy needed
     * ================================================================== */
    {
        struct Point p1 = {10, 20};
        struct Point p2 = {1, 2};
        struct Point sum_pt = fn->add_point(p1, p2);
        printf("add_point((10,20), (1,2)) = (%" PRId32 ", %" PRId32 ")\n",
               sum_pt.x, sum_pt.y);
    }

    printf("Basic lib example passed!\n");
    return 0;
}
