#include <stdio.h>
#include <inttypes.h>
#include "hicc_no_std_demo.h"

int main(void) {
    const struct Hicc_no_std_demo *fn = no_std_demo();

    /* 1. Plain i32 */
    int32_t sum = fn->add(3, 4);
    printf("add(3, 4) = %" PRId32 "\n", sum);

    int32_t neg = fn->negate(5);
    printf("negate(5) = %" PRId32 "\n", neg);

    /* 2. Container<i32> */
    {
        struct AbiClass_Container_i32 c = fn->new_container(42);
        int32_t val = *c.methods->get(&c);
        printf("container_value(42) = %" PRId32 "\n", val);
        c.methods->hicc_destroy(c);
    }

    /* 3. Option<i32> — consume by value transfers ownership to Rust */
    {
        struct AbiClass_Option_i32 opt = fn->new_option(99);
        /* Take a reference before consuming */
        const int32_t *p = opt.methods->as_ref(&opt);
        int64_t doubled = *p * 2LL;
        printf("double_option(Some(99)) = %" PRId64 "\n", doubled);
        opt.methods->hicc_destroy(opt);
    }

    printf("no_std demo passed!\n");
    return 0;
}
