#include <stdio.h>
#include <assert.h>
#include <string.h>
#include "hicc_foreign_type.h"

static void on_return_foreign_string(struct AbiClass_Foreign_String val, const void *ctx) {
    struct AbiClass_Foreign_String *slot = (struct AbiClass_Foreign_String *)ctx;
    *slot = val;
}

int main(void) {
    const struct Hicc_foreign_type *fn = foreign_type();

    puts("=== Lib entry ===");
    printf("foreign_type() at %p\n", (void*)fn);

    /* ===== str methods (built-in: all take AbiClass ptr/value) ===== */
    puts("\n--- str methods ---");
    struct AbiClass_str s = fn->new_str();

    uintptr_t s_len = s.methods->len(&s);
    printf("s.len() = %lu (expect 5)\n", (unsigned long)s_len);
    assert(s_len == 5);

    uintptr_t s_size = s.methods->hicc_size_of();
    printf("s.size_of() = %lu\n", (unsigned long)s_size);

    uint8_t first = s.methods->get(&s, 0);
    printf("s.get(0) = '%c' (expect 'h')\n", (char)first);
    assert(first == 'h');

    struct AbiClass_str s_ref = s.methods->hicc_make_ref(&s);
    uintptr_t s_ref_len = s_ref.methods->len(&s_ref);
    printf("s_ref.len() = %lu (expect 5)\n", (unsigned long)s_ref_len);
    assert(s_ref_len == 5);

    struct AbiClass_str s_mut = s.methods->hicc_make_ref_mut(&s);
    uintptr_t s_mut_len = s_mut.methods->len(&s_mut);
    printf("s_mut.len() = %lu (expect 5)\n", (unsigned long)s_mut_len);
    assert(s_mut_len == 5);

    struct AbiClass_str s_owned = s.methods->hicc_make_unique(s);
    uintptr_t s_owned_len = s_owned.methods->len(&s_owned);
    printf("s_owned.len() = %lu (expect 5)\n", (unsigned long)s_owned_len);
    assert(s_owned_len == 5);
    s_owned.methods->hicc_destroy(s_owned);
    /* make_unique consumed s; get a fresh one */
    s = fn->new_str();

    /* ===== Vec methods (&self -> inner data ptr; &mut self -> AbiClass ptr) ===== */
    puts("\n--- Vec methods ---");
    struct AbiClass_Foreign_Vec_String v = fn->new_vec_string();

    uintptr_t v_size = v.methods->hicc_size_of();
    printf("v.size_of() = %lu\n", (unsigned long)v_size);

    /* len(&self) takes inner data pointer (this_) */
    uintptr_t v_len = v.methods->len(&v);
    printf("v.len() before push = %lu (expect 0)\n", (unsigned long)v_len);
    assert(v_len == 0);

    /* make_ref takes AbiClass ptr */
    struct AbiClass_Foreign_Vec_String v_ref = v.methods->hicc_make_ref(&v);
    uintptr_t v_ref_len = v_ref.methods->len(&v_ref);
    printf("v_ref.len() = %lu (expect 0)\n", (unsigned long)v_ref_len);
    assert(v_ref_len == 0);

    struct AbiClass_Foreign_Vec_String v_mut = v.methods->hicc_make_ref_mut(&v);
    uintptr_t v_mut_len = v_mut.methods->len(&v_mut);
    printf("v_mut.len() = %lu (expect 0)\n", (unsigned long)v_mut_len);
    assert(v_mut_len == 0);

    /* ===== Lib push ===== */
    puts("\n--- push ---");
    struct AbiClass_Foreign_String result = fn->push(&v, s);
    printf("fn->push(&v, s) -> result\n");

    v_len = v.methods->len(&v);
    printf("v.len() after push = %lu (expect 1)\n", (unsigned long)v_len);
    assert(v_len == 1);

    /* ===== String methods ===== */
    puts("\n--- String methods ---");

    uintptr_t result_size = result.methods->hicc_size_of();
    printf("result.size_of() = %lu\n", (unsigned long)result_size);

    /* len(&self) takes inner data ptr */
    uintptr_t result_len = result.methods->len(&result);
    printf("result.len() = %lu (expect 5)\n", (unsigned long)result_len);
    assert(result_len == 5);

    /* as_str(&self) takes inner data ptr, returns AbiClass_str */
    struct AbiClass_str result_str = result.methods->as_str(&result);
    uintptr_t result_str_len = result_str.methods->len(&result_str);
    printf("result.as_str().len() = %lu (expect 5)\n", (unsigned long)result_str_len);
    assert(result_str_len == 5);
    uint8_t result_first = result_str.methods->get(&result_str, 0);
    printf("result.as_str()[0] = '%c' (expect 'h')\n", (char)result_first);
    assert(result_first == 'h');

    /* push_str(&mut self, val) takes AbiClass ptr for self */
    struct AbiClass_str s2 = fn->new_str();
    result.methods->push_str(&result, s2);
    result_len = result.methods->len(&result);
    printf("result.len() after push_str = %lu (expect 10)\n", (unsigned long)result_len);
    assert(result_len == 10);

    /* ===== String.async_len (async associated method via wait) ===== */
    puts("\n--- String.async_len via wait ---");
    struct AbiClass_Box_dynHiccRuntime rt3 = fn->new_runtime();
    struct AbiClass_Box_dynFuture_Output_usize future_alen = result.methods->async_len(&result);
    uintptr_t async_alen_result = future_alen.methods->wait(future_alen, &rt3);
    printf("async_len() = %lu (expect 10)\n", (unsigned long)async_alen_result);
    assert(async_alen_result == 10);
    rt3.methods->hicc_destroy(rt3);

    struct AbiClass_Foreign_String result_ref = result.methods->hicc_make_ref(&result);
    uintptr_t result_ref_len = result_ref.methods->len(&result_ref);
    printf("result_ref.len() = %lu (expect 10)\n", (unsigned long)result_ref_len);
    assert(result_ref_len == 10);

    struct AbiClass_Foreign_String result_owned = result.methods->hicc_make_unique(result);
    uintptr_t result_owned_len = result_owned.methods->len(&result_owned);
    printf("result_owned.len() = %lu (expect 10)\n", (unsigned long)result_owned_len);
    assert(result_owned_len == 10);
    result_owned.methods->hicc_destroy(result_owned);
    /* s2 consumed by push_str; get fresh */
    s2 = fn->new_str();
    /* result consumed by make_unique; get fresh */
    result = fn->push(&v, s2);

    /* ===== Write: overwrite v with a fresh vec ===== */
    puts("\n--- Write ---");
    struct AbiClass_Foreign_Vec_String v_fresh = fn->new_vec_string();
    struct AbiClass_str s3 = fn->new_str();
    struct AbiClass_Foreign_String r3 = fn->push(&v_fresh, s3);
    v.methods->hicc_write(&v, v_fresh);
    v_len = v.methods->len(&v);
    printf("v.len() after write = %lu (expect 1)\n", (unsigned long)v_len);
    assert(v_len == 1);
    r3.methods->hicc_destroy(r3);

    /* ===== Vec.push via method table (&mut self takes AbiClass ptr) ===== */
    puts("\n--- Vec.push via methods ---");
    struct AbiClass_str s4 = fn->new_str();
    struct AbiClass_Foreign_String s4_string = fn->push(&v, s4);
    v_len = v.methods->len(&v);
    printf("v.len() after fn->push = %lu (expect 2)\n", (unsigned long)v_len);
    assert(v_len == 2);

    /* Push s4_string again via method table to demonstrate methods->push */
    v.methods->push(&v, s4_string);
    v_len = v.methods->len(&v);
    printf("v.len() after methods->push = %lu (expect 3)\n", (unsigned long)v_len);
    assert(v_len == 3);

    /* ===== is_foreign() inline method verification ===== */
    puts("\n--- is_foreign() ---");
    bool v_is_foreign = v.methods->is_foreign(&v);
    printf("v.is_foreign() = %d (expect 1)\n", v_is_foreign);
    assert(v_is_foreign == 1);
    bool result_is_foreign = result.methods->is_foreign(&result);
    printf("result.is_foreign() = %d (expect 1)\n", result_is_foreign);
    assert(result_is_foreign == 1);

    /* ===== HashSet<Foreign<String>>: new_set_foreign_string + process_foreign_set ===== */
    puts("\n--- HashSet<Foreign<String>> ---");
    struct AbiClass_HashSet_Foreign_String hs = fn->new_set_foreign_string();

    uintptr_t hs_len = hs.methods->len(&hs);
    printf("hs.len() after new_set_foreign_string = %lu (expect 0)\n", (unsigned long)hs_len);
    assert(hs_len == 0);

    /* Create a Foreign_String and insert it (uses existing v which is still alive) */
    struct AbiClass_str s5 = fn->new_str();
    struct AbiClass_Foreign_String s5_fs = fn->push(&v, s5);
    bool inserted = hs.methods->insert(&hs, s5_fs);
    printf("hs.insert(s5_fs) = %d (expect 1)\n", inserted);
    assert(inserted == 1);

    hs_len = hs.methods->len(&hs);
    printf("hs.len() after insert = %lu (expect 1)\n", (unsigned long)hs_len);
    assert(hs_len == 1);

    /* Call process_foreign_set which adds "from_process" */
    hs = fn->process_foreign_set(hs);
    hs_len = hs.methods->len(&hs);
    printf("hs.len() after process_foreign_set = %lu (expect 2)\n", (unsigned long)hs_len);
    assert(hs_len == 2);

    /* Verify actual values via contains() using make_foreign_string */
    struct AbiClass_Foreign_String check_val;

    /* "hello" should be in the set */
    check_val = fn->make_foreign_string("hello");
    bool found_hello = hs.methods->contains(&hs, &check_val);
    printf("hs.contains(\"hello\") = %d (expect 1)\n", found_hello);
    assert(found_hello == 1);
    check_val.methods->hicc_destroy(check_val);

    /* "from_process" (added by process_foreign_set) should be in the set */
    check_val = fn->make_foreign_string("from_process");
    bool found_from_process = hs.methods->contains(&hs, &check_val);
    printf("hs.contains(\"from_process\") = %d (expect 1)\n", found_from_process);
    assert(found_from_process == 1);
    check_val.methods->hicc_destroy(check_val);

    /* "world" should NOT be in the set */
    check_val = fn->make_foreign_string("world");
    bool found_world = hs.methods->contains(&hs, &check_val);
    printf("hs.contains(\"world\") = %d (expect 0)\n", found_world);
    assert(found_world == 0);
    check_val.methods->hicc_destroy(check_val);

    hs.methods->hicc_destroy(hs);

    /* ===== Final cleanup ===== */
    result.methods->hicc_destroy(result);
    v.methods->hicc_destroy(v);

    /* ===== Async: make_name via wait (blocking) ===== */
    puts("\n--- Async: make_name via wait ---");
    struct AbiClass_Box_dynHiccRuntime rt = fn->new_runtime();
    printf("Runtime created\n");

    struct AbiClass_Box_dynFuture_Output_Foreign_String future1 = fn->make_name("hello");
    struct AbiClass_Foreign_String async_result = future1.methods->wait(future1, &rt);
    uintptr_t async_len = async_result.methods->len(&async_result);
    fflush(stdout);
    printf("make_name(\"hello\") len = %lu (expect 11)\n", (unsigned long)async_len);
    fflush(stdout);
    printf("async_result.methods = %p, this_ = %p, level = %lu\n", (void*)async_result.methods, async_result.this_, (unsigned long)async_result.level);
    fflush(stdout);
    assert(async_len == 11);
    struct AbiClass_str async_str = async_result.methods->as_str(&async_result);
    uint8_t async_first = async_str.methods->get(&async_str, 0);
    printf("make_name(\"hello\")[0] = '%c' (expect 'h')\n", (char)async_first);
    assert(async_first == 'h');
    async_str.methods->hicc_destroy(async_str);
    async_result.methods->hicc_destroy(async_result);

    /* ===== Async: make_name via async_wait (callback) ===== */
    puts("\n--- Async: make_name via async_wait ---");
    struct AbiClass_Box_dynHiccRuntime rt2 = fn->new_runtime();
    struct AbiClass_Box_dynFuture_Output_Foreign_String future2 = fn->make_name("async");
    struct AbiClass_Foreign_String result_slot = {0};
    Notify_Foreign_String notify2;
    notify2.on_return = on_return_foreign_string;
    notify2.ctx = &result_slot;
    future2.methods->async_wait(future2, &rt2, notify2);
    uintptr_t async_len2 = result_slot.methods->len(&result_slot);
    printf("make_name(\"async\") len = %lu (expect 11)\n", (unsigned long)async_len2);
    assert(async_len2 == 11);
    result_slot.methods->hicc_destroy(result_slot);
    rt2.methods->hicc_destroy(rt2);

    rt.methods->hicc_destroy(rt);

    puts("\nForeign type example passed!");
    return 0;
}
