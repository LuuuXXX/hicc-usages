#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "hicc_async_func.h"

/* Callback for async_wait with i32 result */
static void on_return_i32(int32_t result, const void *ctx) {
    int32_t *slot = (int32_t *)ctx;
    *slot = result;
}

/* Callback for async_wait with String result */
static void on_return_string(struct AbiClass_String result, const void *ctx) {
    /* Store the AbiClass_String in the slot; caller will extract value */
    struct AbiClass_String *slot = (struct AbiClass_String *)ctx;
    *slot = result;
}

int main(void) {
    const struct Hicc_async_func *fn = async_func();

    puts("=== Async-func example ===");

    /* Create runtime (Box<dyn HiccRuntime>) */
    puts("\n--- Create runtime ---");
    struct AbiClass_Box_dynHiccRuntime rt = fn->new_runtime();
    printf("Runtime created: methods=%p, this=%p, level=%lu\n",
           (void*)rt.methods, rt.this_, (unsigned long)rt.level);

    /* ===== Test 1: async_add via wait (blocking) ===== */
    puts("\n--- async_add(3, 4) via wait ---");
    struct AbiClass_Box_dynFuture_Output_Foreign_i32 future_add = fn->async_add(3, 4);
    int32_t result_add = future_add.methods->wait(future_add, &rt);
    printf("async_add(3, 4) = %d (expect 7)\n", result_add);
    assert(result_add == 7);

    /* ===== Test 2: async_add via async_wait (callback) ===== */
    puts("\n--- async_add(10, 20) via async_wait ---");
    struct AbiClass_Box_dynFuture_Output_Foreign_i32 future_add2 = fn->async_add(10, 20);
    int32_t result_add2 = 0;
    struct Notify_Foreign_i32 notify_add2 = {
        .on_return = on_return_i32,
        .ctx = (const void *)&result_add2
    };
    future_add2.methods->async_wait(future_add2, &rt, notify_add2);
    printf("async_add(10, 20) = %d (expect 30)\n", result_add2);
    assert(result_add2 == 30);

    /* ===== Test 3: async_hello via wait (blocking) ===== */
    puts("\n--- async_hello via wait ---");
    struct AbiClass_Box_dynFuture_Output_Foreign_String future_hello = fn->async_hello();
    struct AbiClass_String result_str = future_hello.methods->wait(future_hello, &rt);
    uintptr_t str_len = result_str.methods->len(&result_str);
    printf("async_hello().len() = %lu (expect 11)\n", (unsigned long)str_len);
    assert(str_len == 11);
    /* Read the string content via as_str */
    struct AbiClass_str str_ref = result_str.methods->as_str(&result_str);
    uint8_t first_char = str_ref.methods->get(&str_ref, 0);
    printf("async_hello()[0] = '%c' (expect 'h')\n", (char)first_char);
    assert(first_char == 'h');
    result_str.methods->hicc_destroy(result_str);

    /* ===== Test 4: async_hello via async_wait (callback) ===== */
    puts("\n--- async_hello via async_wait ---");
    struct AbiClass_Box_dynFuture_Output_Foreign_String future_hello2 = fn->async_hello();
    struct AbiClass_String result_str2;
    memset(&result_str2, 0, sizeof(result_str2));
    struct Notify_Foreign_String notify_str2 = {
        .on_return = on_return_string,
        .ctx = (const void *)&result_str2
    };
    future_hello2.methods->async_wait(future_hello2, &rt, notify_str2);
    uintptr_t str_len2 = result_str2.methods->len(&result_str2);
    printf("async_hello (async_wait).len() = %lu (expect 11)\n", (unsigned long)str_len2);
    assert(str_len2 == 11);
    result_str2.methods->hicc_destroy(result_str2);

    /* ===== Test 5: AsyncCounter.async_increment via wait ===== */
    puts("\n--- AsyncCounter.async_increment(5) via wait ---");
    struct AbiClass_AsyncCounter counter = fn->new_counter(100);
    struct AbiClass_Box_dynFuture_Output_i32 future_inc =
        counter.methods->async_increment(&counter, 5);
    int32_t result_inc = future_inc.methods->wait(future_inc, &rt);
    printf("async_increment(5) = %d (expect 105)\n", result_inc);
    assert(result_inc == 105);
    counter.methods->hicc_destroy(counter);

    /* ===== Test 6: AsyncCounter.async_greet via async_wait ===== */
    puts("\n--- AsyncCounter.async_greet via async_wait ---");
    struct AbiClass_AsyncCounter counter2 = fn->new_counter(42);
    struct AbiClass_Box_dynFuture_Output_String future_greet =
        counter2.methods->async_greet(&counter2);
    struct AbiClass_String result_greet_str;
    memset(&result_greet_str, 0, sizeof(result_greet_str));
    struct Notify_String notify_greet = {
        .on_return = on_return_string,
        .ctx = (const void *)&result_greet_str
    };
    future_greet.methods->async_wait(future_greet, &rt, notify_greet);
    uintptr_t greet_len = result_greet_str.methods->len(&result_greet_str);
    printf("async_greet().len() = %lu (expect 13)\n", (unsigned long)greet_len);
    assert(greet_len == 13);
    result_greet_str.methods->hicc_destroy(result_greet_str);
    counter2.methods->hicc_destroy(counter2);

    /* ===== Cleanup: destroy runtime ===== */
    puts("\n--- Cleanup ---");
    rt.methods->hicc_destroy(rt);
    printf("Runtime destroyed\n");

    puts("\nAsync-func example passed!");
    return 0;
}