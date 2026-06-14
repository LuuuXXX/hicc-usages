#include <stdio.h>
#include <assert.h>
#include <inttypes.h>
#include "hicc_rust_std.h"

int main(void) {
    const struct Hicc_rust_std *fn = rust_std();
    printf("rust_std() at %p\n", (void*)fn);

    /* ===== Option<i32> ===== */
    puts("\n--- Option<i32> ---");
    {
        struct AbiClass_Option_i32 opt = fn->new_option_i32();
        bool is_none = opt.methods->is_none(&opt);
        printf("Option<i32>::is_none() = %s (expect false)\n", is_none ? "true" : "false");
        assert(!is_none);

        uintptr_t sz = opt.methods->hicc_size_of();
        printf("Option<i32>::size_of() = %lu\n", (unsigned long)sz);

        const int32_t *p = opt.methods->as_ref(&opt);
        printf("Option<i32>::as_ref() = %d (expect 42)\n", *p);
        assert(*p == 42);

        int32_t val = opt.methods->unwrap(opt);
        printf("Option<i32>::unwrap() = %d (expect 42)\n", val);
        assert(val == 42);
    }

    /* ===== Option<i32> None ===== */
    puts("\n--- Option<i32> None ---");
    {
        struct AbiClass_Option_i32 none = fn->new_option_none_i32();
        bool is_none = none.methods->is_none(&none);
        printf("Option<i32>::is_none() = %s (expect true)\n", is_none ? "true" : "false");
        assert(is_none);
        none.methods->hicc_destroy(none);
    }

    /* ===== Option<String> ===== */
    puts("\n--- Option<String> ---");
    {
        struct AbiClass_Option_String opts = fn->new_option_string();
        bool is_none = opts.methods->is_none(&opts);
        assert(!is_none);

        struct AbiClass_String inner = opts.methods->as_ref(&opts);
        uintptr_t slen = inner.methods->len(&inner);
        printf("Option<String>::as_ref()->len() = %lu (expect 5)\n", (unsigned long)slen);
        assert(slen == 5);
        inner.methods->hicc_destroy(inner);

        struct AbiClass_Option_String taken = opts.methods->take(&opts);
        bool taken_none = taken.methods->is_none(&taken);
        assert(!taken_none);
        struct AbiClass_String unwrapped = taken.methods->unwrap(taken);
        uintptr_t ulen = unwrapped.methods->len(&unwrapped);
        printf("Option<String>::take()->unwrap()->len() = %lu (expect 5)\n", (unsigned long)ulen);
        assert(ulen == 5);
        unwrapped.methods->hicc_destroy(unwrapped);

        bool orig_none = opts.methods->is_none(&opts);
        printf("After take, original is_none = %s (expect true)\n", orig_none ? "true" : "false");
        assert(orig_none);
        opts.methods->hicc_destroy(opts);
    }

    /* ===== Result<i32, bool> Ok ===== */
    puts("\n--- Result<i32, bool> Ok ---");
    {
        struct AbiClass_Result_i32_bool ok = fn->new_result_ok_i32();
        assert(ok.methods->is_ok(&ok));
        assert(!ok.methods->is_err(&ok));
        int32_t val = ok.methods->ok(ok);
        printf("Result::ok() = %d (expect 100)\n", val);
        assert(val == 100);
    }

    /* ===== Result<i32, bool> Err ===== */
    puts("\n--- Result<i32, bool> Err ---");
    {
        struct AbiClass_Result_i32_bool err = fn->new_result_err_bool();
        assert(err.methods->is_err(&err));
        assert(!err.methods->is_ok(&err));
        bool val = err.methods->err(err);
        printf("Result::err() = %s (expect false)\n", val ? "true" : "false");
        assert(!val);
    }

    /* ===== Array<Option<i32>, 3> ===== */
    puts("\n--- Array<Option<i32>, 3> ---");
    {
        struct AbiClass_Option_i32_3 arr = fn->new_array_option_i32();
        uintptr_t alen = arr.methods->len(&arr);
        printf("Array::len() = %lu (expect 3)\n", (unsigned long)alen);
        assert(alen == 3);

        struct AbiClass_Option_i32 item0 = arr.methods->get(&arr, 0);
        bool item0_none = item0.methods->is_none(&item0);
        assert(!item0_none);
        const int32_t *v0 = item0.methods->as_ref(&item0);
        printf("Array::get(0)->as_ref() = %d (expect 10)\n", *v0);
        assert(*v0 == 10);
        item0.methods->hicc_destroy(item0);

        arr.methods->set(&arr, 0, fn->new_option_i32());
        struct AbiClass_Option_i32 new0 = arr.methods->get(&arr, 0);
        const int32_t *nv0 = new0.methods->as_ref(&new0);
        printf("Array::set(0,Some(42))::get(0)->as_ref() = %d (expect 42)\n", *nv0);
        assert(*nv0 == 42);
        new0.methods->hicc_destroy(new0);

        struct AbiClass_Option_i32 item1 = arr.methods->get(&arr, 1);
        assert(item1.methods->is_none(&item1));
        item1.methods->hicc_destroy(item1);

        arr.methods->hicc_destroy(arr);
    }

    /* ===== Tuple (i32, i32) ===== */
    puts("\n--- Tuple (i32, i32) ---");
    {
        struct AbiClass_i32_i32 t = fn->new_tuple_i32_i32();
        const int32_t *f0 = t.methods->field_0(&t);
        const int32_t *f1 = t.methods->field_1(&t);
        printf("Tuple::field_0() = %d, field_1() = %d (expect 1, 2)\n", *f0, *f1);
        assert(*f0 == 1 && *f1 == 2);

        int32_t v0 = t.methods->take_0(t);
        printf("Tuple::take_0() = %d (expect 1)\n", v0);
        assert(v0 == 1);
    }

    /* ===== Tuple (i32, String) ===== */
    puts("\n--- Tuple (i32, String) ---");
    {
        struct AbiClass_i32_String ts = fn->new_tuple_i32_string();
        const int32_t *tf0 = ts.methods->field_0(&ts);
        printf("Tuple::field_0() = %d (expect 99)\n", *tf0);
        assert(*tf0 == 99);

        struct AbiClass_String tf1 = ts.methods->field_1(&ts);
        uintptr_t tflen = tf1.methods->len(&tf1);
        printf("Tuple::field_1()->len() = %lu (expect 5)\n", (unsigned long)tflen);
        assert(tflen == 5);
        tf1.methods->hicc_destroy(tf1);

        int32_t tv0 = ts.methods->take_0(ts);
        printf("Tuple::take_0() = %d (expect 99)\n", tv0);
        assert(tv0 == 99);
    }

    /* ===== Tuple3-6 (field_N + take_N) ===== */
    puts("\n--- Tuple3..6 ---");
    {
        struct AbiClass_i32_i32_i32 t3 = fn->new_tuple_i32_i32_i32();
        assert(*t3.methods->field_0(&t3) == 1);
        assert(*t3.methods->field_1(&t3) == 2);
        assert(*t3.methods->field_2(&t3) == 3);
        printf("Tuple3::field_0/1/2 = 1/2/3 OK\n");
        assert(t3.methods->take_0(t3) == 1);
    }
    {
        struct AbiClass_i32_i32_i32_i32 t4 = fn->new_tuple_i32_i32_i32_i32();
        assert(*t4.methods->field_0(&t4) == 1);
        assert(*t4.methods->field_1(&t4) == 2);
        assert(*t4.methods->field_2(&t4) == 3);
        assert(*t4.methods->field_3(&t4) == 4);
        printf("Tuple4::field_0/1/2/3 = 1/2/3/4 OK\n");
        assert(t4.methods->take_0(t4) == 1);
    }
    {
        struct AbiClass_i32_i32_i32_i32_i32 t5 = fn->new_tuple_i32_i32_i32_i32_i32();
        assert(*t5.methods->field_0(&t5) == 1);
        assert(*t5.methods->field_4(&t5) == 5);
        printf("Tuple5::field_0/4 = 1/5 OK\n");
        assert(t5.methods->take_0(t5) == 1);
    }
    {
        struct AbiClass_i32_i32_i32_i32_i32_i32 t6 = fn->new_tuple_i32_i32_i32_i32_i32_i32();
        assert(*t6.methods->field_0(&t6) == 1);
        assert(*t6.methods->field_5(&t6) == 6);
        printf("Tuple6::field_0/5 = 1/6 OK\n");
        assert(t6.methods->take_0(t6) == 1);
    }

    /* ===== &str ===== */
    puts("\n--- &str ---");
    {
        struct AbiClass_str s = fn->new_str();
        uintptr_t slen = s.methods->len(&s);
        printf("str::len() = %lu (expect 5)\n", (unsigned long)slen);
        assert(slen == 5);

        uint8_t ch = s.methods->get(&s, 0);
        printf("str::get(0) = '%c' (expect 'h')\n", (char)ch);
        assert(ch == 'h');

        s.methods->hicc_destroy(s);
    }

    /* ===== Vec<i32> ===== */
    puts("\n--- Vec<i32> ---");
    {
        struct AbiClass_Vec_i32 v = fn->new_vec_i32();
        assert(v.methods->len(&v) == 0);

        v.methods->push(&v, 10);
        v.methods->push(&v, 20);
        v.methods->push(&v, 30);
        printf("Vec<i32>::len() after 3 pushes = %lu (expect 3)\n", (unsigned long)v.methods->len(&v));
        assert(v.methods->len(&v) == 3);

        const int32_t *g0 = v.methods->get(&v, 0);
        printf("Vec<i32>::get(0) = %d (expect 10)\n", *g0);
        assert(*g0 == 10);

        int32_t *gm1 = v.methods->get_mut(&v, 1);
        *gm1 = 200;
        const int32_t *g1 = v.methods->get(&v, 1);
        printf("Vec<i32>::get_mut(1)=200, get(1) = %d (expect 200)\n", *g1);
        assert(*g1 == 200);

        struct AbiClass_Option_i32 popped = v.methods->pop(&v);
        int32_t pval = popped.methods->unwrap(popped);
        printf("Vec<i32>::pop() = %d (expect 30)\n", pval);
        assert(pval == 30);

        struct AbiClass_i32 slice = v.methods->as_slice(&v);
        printf("Vec<i32>::as_slice()->len() = %lu (expect 2)\n", (unsigned long)slice.methods->len(&slice));
        assert(slice.methods->len(&slice) == 2);
        slice.methods->hicc_destroy(slice);

        v.methods->hicc_destroy(v);
    }

    /* ===== Vec<String> ===== */
    puts("\n--- Vec<String> ---");
    {
        struct AbiClass_Vec_String vs = fn->new_vec_string();
        assert(vs.methods->len(&vs) == 0);

        struct AbiClass_String s1 = fn->new_string();
        vs.methods->push(&vs, s1);
        printf("Vec<String>::len() after push = %lu (expect 1)\n", (unsigned long)vs.methods->len(&vs));
        assert(vs.methods->len(&vs) == 1);

        struct AbiClass_String g0s = vs.methods->get(&vs, 0);
        printf("Vec<String>::get(0)->len() = %lu (expect 5)\n", (unsigned long)g0s.methods->len(&g0s));
        assert(g0s.methods->len(&g0s) == 5);
        g0s.methods->hicc_destroy(g0s);

        struct AbiClass_string_String sls = vs.methods->as_slice(&vs);
        printf("Vec<String>::as_slice()->len() = %lu (expect 1)\n", (unsigned long)sls.methods->len(&sls));
        assert(sls.methods->len(&sls) == 1);
        sls.methods->hicc_destroy(sls);

        struct AbiClass_Option_String ps = vs.methods->pop(&vs);
        struct AbiClass_String pv = ps.methods->unwrap(ps);
        printf("Vec<String>::pop()->unwrap()->len() = %lu (expect 5)\n", (unsigned long)pv.methods->len(&pv));
        assert(pv.methods->len(&pv) == 5);
        pv.methods->hicc_destroy(pv);

        vs.methods->hicc_destroy(vs);
    }

    /* ===== String ===== */
    puts("\n--- String ---");
    {
        struct AbiClass_String s = fn->new_string();
        assert(s.methods->len(&s) == 5);

        struct AbiClass_str sr = s.methods->as_str(&s);
        assert(sr.methods->len(&sr) == 5);
        sr.methods->hicc_destroy(sr);

        struct AbiClass_u8 sb = s.methods->as_bytes(&s);
        assert(sb.methods->len(&sb) == 5);
        sb.methods->hicc_destroy(sb);

        /* push_str consumes the push_arg (AbiClass_str passed by value = ownership transfer) */
        struct AbiClass_str push_arg1 = fn->new_str();
        s.methods->push_str(&s, push_arg1);
        /* push_arg1 is consumed - DO NOT reuse or destroy */
        printf("String::push_str()->len() = %lu (expect 10)\n", (unsigned long)s.methods->len(&s));
        assert(s.methods->len(&s) == 10);

        s.methods->push_cstr(&s, (const int8_t *)" world");
        printf("String::push_cstr()->len() = %lu (expect 16)\n", (unsigned long)s.methods->len(&s));
        assert(s.methods->len(&s) == 16);

        /* insert_str also consumes its AbiClass_str argument */
        struct AbiClass_str push_arg2 = fn->new_str();
        s.methods->insert_str(&s, 5, push_arg2);
        /* push_arg2 is consumed - DO NOT reuse or destroy */
        s.methods->insert_cstr(&s, 10, (const int8_t *)" beautiful");
        printf("String after insert_str+insert_cstr len = %lu\n", (unsigned long)s.methods->len(&s));

        s.methods->hicc_destroy(s);
    }

    /* ===== String empty + push_cstr/insert_cstr ===== */
    puts("\n--- String (empty) ---");
    {
        struct AbiClass_String se = fn->new_string_empty();
        assert(se.methods->len(&se) == 0);

        se.methods->push_cstr(&se, (const int8_t *)"hello");
        printf("empty String::push_cstr()->len() = %lu (expect 5)\n", (unsigned long)se.methods->len(&se));
        assert(se.methods->len(&se) == 5);

        se.methods->insert_cstr(&se, 5, (const int8_t *)" world");
        printf("String::insert_cstr(5,\" world\")->len() = %lu (expect 11)\n", (unsigned long)se.methods->len(&se));
        assert(se.methods->len(&se) == 11);

        se.methods->hicc_destroy(se);
    }

    /* ===== Box<i32> ===== */
    puts("\n--- Box<i32> ---");
    {
        struct AbiClass_Box_i32 b = fn->new_box_i32();
        const int32_t *bg = b.methods->get(&b);
        printf("Box::get() = %d (expect 42)\n", *bg);
        assert(*bg == 42);

        int32_t *bm = b.methods->get_mut(&b);
        *bm = 100;
        const int32_t *bg2 = b.methods->get(&b);
        printf("Box::get_mut()=100, get() = %d (expect 100)\n", *bg2);
        assert(*bg2 == 100);

        b.methods->hicc_destroy(b);
    }

    /* ===== Box<String> ===== */
    puts("\n--- Box<String> ---");
    {
        /* Box<String> 的 Abi 类型是 AbiClass_String，可直接调用 String 的方法 */
        struct AbiClass_String bs = fn->new_box_string();
        uintptr_t bslen = bs.methods->len(&bs);
        printf("Box<String>::len() = %lu (expect 5)\n", (unsigned long)bslen);
        assert(bslen == 5);

        struct AbiClass_str bsr = bs.methods->as_str(&bs);
        assert(bsr.methods->len(&bsr) == 5);
        bsr.methods->hicc_destroy(bsr);

        bs.methods->push_cstr(&bs, (const int8_t *)" world");
        printf("Box<String>::push_cstr()->len() = %lu (expect 11)\n", (unsigned long)bs.methods->len(&bs));
        assert(bs.methods->len(&bs) == 11);

        bs.methods->hicc_destroy(bs);
    }

    /* ===== Rc<i32> ===== */
    puts("\n--- Rc<i32> ---");
    {
        struct AbiClass_Rc_i32 rc = fn->new_rc_i32();
        const int32_t *rvg = rc.methods->get(&rc);
        printf("Rc::get() = %d (expect 42)\n", *rvg);
        assert(*rvg == 42);
        rc.methods->hicc_destroy(rc);
    }

    /* ===== Arc<i32> ===== */
    puts("\n--- Arc<i32> ---");
    {
        struct AbiClass_Arc_i32 arc = fn->new_arc_i32();
        const int32_t *avg = arc.methods->get(&arc);
        printf("Arc::get() = %d (expect 42)\n", *avg);
        assert(*avg == 42);
        arc.methods->hicc_destroy(arc);
    }

    /* ===== BTreeMap<i32, i32> ===== */
    puts("\n--- BTreeMap<i32, i32> ---");
    {
        struct AbiClass_BTreeMap_i32_i32 bm = fn->new_btreemap_i32_i32();
        assert(bm.methods->is_empty(&bm));
        assert(bm.methods->len(&bm) == 0);

        struct AbiClass_Option_i32 ins_ret = bm.methods->insert(&bm, 1, 100);
        bool ins_none = ins_ret.methods->is_none(&ins_ret);
        assert(ins_none);
        ins_ret.methods->hicc_destroy(ins_ret);

        bm.methods->insert(&bm, 2, 200);
        bm.methods->insert(&bm, 3, 300);
        printf("BTreeMap::len() after 3 inserts = %lu (expect 3)\n", (unsigned long)bm.methods->len(&bm));
        assert(bm.methods->len(&bm) == 3);

        assert(bm.methods->contains_key(&bm, &((int32_t){1})));
        assert(!bm.methods->contains_key(&bm, &((int32_t){99})));

        struct AbiClass_option_Option_i32 gv = bm.methods->get(&bm, &((int32_t){1}));
        const int32_t *const *gvval_ptr = gv.methods->as_ref(&gv);
        const int32_t *gvval = *gvval_ptr;
        printf("BTreeMap::get(1)->as_ref() = %d (expect 100)\n", *gvval);
        assert(*gvval == 100);
        gv.methods->hicc_destroy(gv);

        struct AbiClass_Option_i32 rm_ret = bm.methods->remove(&bm, &((int32_t){1}));
        int32_t rm_val = rm_ret.methods->unwrap(rm_ret);
        printf("BTreeMap::remove(1) = %d (expect 100)\n", rm_val);
        assert(rm_val == 100);
        assert(bm.methods->len(&bm) == 2);

        struct AbiClass_Iter_i32_i32 iter = bm.methods->iter(&bm);
        int count = 0;
        while (true) {
            struct AbiClass_Option_i32_i32 entry = iter.methods->next(&iter);
            if (entry.methods->is_none(&entry)) { entry.methods->hicc_destroy(entry); break; }
            count++;
            entry.methods->hicc_destroy(entry);
        }
        printf("BTreeMap::iter() count = %d (expect 2)\n", count);
        assert(count == 2);
        iter.methods->hicc_destroy(iter);

        bm.methods->hicc_destroy(bm);
    }

    /* ===== BTreeMap<String, String> ===== */
    puts("\n--- BTreeMap<String, String> ---");
    {
        struct AbiClass_BTreeMap_String_String bms = fn->new_btreemap_string_string();
        assert(bms.methods->is_empty(&bms));

        struct AbiClass_String k1 = fn->new_string();
        struct AbiClass_String v1 = fn->new_string();
        struct AbiClass_Option_String ins1 = bms.methods->insert(&bms, k1, v1);
        assert(ins1.methods->is_none(&ins1));
        ins1.methods->hicc_destroy(ins1);

        struct AbiClass_String k2 = fn->new_string_empty();
        k2.methods->push_cstr(&k2, (const int8_t *)"key2");
        struct AbiClass_String v2 = fn->new_string_empty();
        v2.methods->push_cstr(&v2, (const int8_t *)"val2");
        struct AbiClass_Option_String ins2 = bms.methods->insert(&bms, k2, v2);
        assert(ins2.methods->is_none(&ins2));
        ins2.methods->hicc_destroy(ins2);

        printf("BTreeMap<String>::len() = %lu (expect 2)\n", (unsigned long)bms.methods->len(&bms));
        assert(bms.methods->len(&bms) == 2);

        struct AbiClass_Iter_String_String iters = bms.methods->iter(&bms);
        int scount = 0;
        while (true) {
            struct AbiClass_Option_String_String ent = iters.methods->next(&iters);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            scount++;
            ent.methods->hicc_destroy(ent);
        }
        printf("BTreeMap<String>::iter() count = %d (expect 2)\n", scount);
        assert(scount == 2);
        iters.methods->hicc_destroy(iters);

        bms.methods->hicc_destroy(bms);
    }

    /* ===== BTreeSet<i32> ===== */
    puts("\n--- BTreeSet<i32> ---");
    {
        struct AbiClass_BTreeSet_i32 bs = fn->new_btreeset_i32();
        assert(bs.methods->is_empty(&bs));

        bool inserted = bs.methods->insert(&bs, 10);
        printf("BTreeSet::insert(10) = %s (expect true)\n", inserted ? "true" : "false");
        assert(inserted);
        bs.methods->insert(&bs, 20);
        bs.methods->insert(&bs, 30);
        assert(bs.methods->len(&bs) == 3);

        assert(bs.methods->contains(&bs, &((int32_t){10})));
        assert(!bs.methods->contains(&bs, &((int32_t){99})));

        bool removed = bs.methods->remove(&bs, &((int32_t){10}));
        printf("BTreeSet::remove(10) = %s (expect true)\n", removed ? "true" : "false");
        assert(removed);
        assert(bs.methods->len(&bs) == 2);

        struct AbiClass_Iter_i32 bsiter = bs.methods->iter(&bs);
        int bscount = 0;
        while (true) {
            struct AbiClass_option_Option_i32 e = bsiter.methods->next(&bsiter);
            if (e.methods->is_none(&e)) { e.methods->hicc_destroy(e); break; }
            bscount++;
            e.methods->hicc_destroy(e);
        }
        printf("BTreeSet::iter() count = %d (expect 2)\n", bscount);
        assert(bscount == 2);
        bsiter.methods->hicc_destroy(bsiter);

        bs.methods->hicc_destroy(bs);
    }

    /* ===== BTreeSet<String> ===== */
    puts("\n--- BTreeSet<String> ---");
    {
        struct AbiClass_BTreeSet_String bss = fn->new_btreeset_string();
        assert(bss.methods->is_empty(&bss));

        struct AbiClass_String sk1 = fn->new_string_empty();
        sk1.methods->push_cstr(&sk1, (const int8_t *)"abc");
        bool ins = bss.methods->insert(&bss, sk1);
        assert(ins);
        printf("BTreeSet<String>::insert() = %s\n", ins ? "true" : "false");

        struct AbiClass_Iter_String bsi = bss.methods->iter(&bss);
        int bsc = 0;
        while (true) {
            struct AbiClass_option_Option_string_String e = bsi.methods->next(&bsi);
            if (e.methods->is_none(&e)) { e.methods->hicc_destroy(e); break; }
            bsc++;
            e.methods->hicc_destroy(e);
        }
        printf("BTreeSet<String>::iter() count = %d (expect 1)\n", bsc);
        assert(bsc == 1);
        bsi.methods->hicc_destroy(bsi);

        bss.methods->hicc_destroy(bss);
    }

    /* ===== HashMap<i32, i32> ===== */
    puts("\n--- HashMap<i32, i32> ---");
    {
        struct AbiClass_HashMap_i32_i32 hm = fn->new_hashmap_i32_i32();
        assert(hm.methods->is_empty(&hm));

        hm.methods->insert(&hm, 1, 100);
        hm.methods->insert(&hm, 2, 200);
        printf("HashMap::len() = %lu (expect 2)\n", (unsigned long)hm.methods->len(&hm));
        assert(hm.methods->len(&hm) == 2);

        assert(hm.methods->contains_key(&hm, &((int32_t){1})));

        struct AbiClass_option_Option_i32 hgv = hm.methods->get(&hm, &((int32_t){1}));
        const int32_t *const *hgval_ptr = hgv.methods->as_ref(&hgv);
        const int32_t *hgval = *hgval_ptr;
        printf("HashMap::get(1) = %d (expect 100)\n", *hgval);
        assert(*hgval == 100);
        hgv.methods->hicc_destroy(hgv);

        struct AbiClass_Option_i32 hrm = hm.methods->remove(&hm, &((int32_t){1}));
        int32_t hrmv = hrm.methods->unwrap(hrm);
        printf("HashMap::remove(1) = %d (expect 100)\n", hrmv);
        assert(hrmv == 100);

        struct AbiClass_map_Iter_i32_i32 hiter = hm.methods->iter(&hm);
        int hc = 0;
        while (true) {
            struct AbiClass_Option_i32_i32 he = hiter.methods->next(&hiter);
            if (he.methods->is_none(&he)) { he.methods->hicc_destroy(he); break; }
            hc++;
            he.methods->hicc_destroy(he);
        }
        printf("HashMap::iter() count = %d (expect 1)\n", hc);
        assert(hc == 1);
        hiter.methods->hicc_destroy(hiter);

        hm.methods->hicc_destroy(hm);
    }

    /* ===== HashMap<String, String> ===== */
    puts("\n--- HashMap<String, String> ---");
    {
        struct AbiClass_HashMap_String_String hms = fn->new_hashmap_string_string();
        assert(hms.methods->is_empty(&hms));

        struct AbiClass_String hk = fn->new_string_empty();
        hk.methods->push_cstr(&hk, (const int8_t *)"k1");
        struct AbiClass_String hv = fn->new_string_empty();
        hv.methods->push_cstr(&hv, (const int8_t *)"v1");
        struct AbiClass_Option_String hins = hms.methods->insert(&hms, hk, hv);
        assert(hins.methods->is_none(&hins));
        hins.methods->hicc_destroy(hins);

        printf("HashMap<String>::len() = %lu (expect 1)\n", (unsigned long)hms.methods->len(&hms));
        assert(hms.methods->len(&hms) == 1);

        struct AbiClass_map_Iter_string_String_string_String hsi = hms.methods->iter(&hms);
        int hsc = 0;
        while (true) {
            struct AbiClass_Option_String_String he = hsi.methods->next(&hsi);
            if (he.methods->is_none(&he)) { he.methods->hicc_destroy(he); break; }
            hsc++;
            he.methods->hicc_destroy(he);
        }
        printf("HashMap<String>::iter() count = %d (expect 1)\n", hsc);
        assert(hsc == 1);
        hsi.methods->hicc_destroy(hsi);

        hms.methods->hicc_destroy(hms);
    }

    /* ===== HashSet<i32> ===== */
    puts("\n--- HashSet<i32> ---");
    {
        struct AbiClass_HashSet_i32 hs = fn->new_hashset_i32();
        assert(hs.methods->is_empty(&hs));

        bool hins = hs.methods->insert(&hs, 42);
        assert(hins);
        hs.methods->insert(&hs, 43);
        printf("HashSet::len() = %lu (expect 2)\n", (unsigned long)hs.methods->len(&hs));
        assert(hs.methods->len(&hs) == 2);

        assert(hs.methods->contains(&hs, &((int32_t){42})));

        struct AbiClass_set_Iter_i32 hsi = hs.methods->iter(&hs);
        int hsc = 0;
        while (true) {
            struct AbiClass_option_Option_i32 he = hsi.methods->next(&hsi);
            if (he.methods->is_none(&he)) { he.methods->hicc_destroy(he); break; }
            hsc++;
            he.methods->hicc_destroy(he);
        }
        printf("HashSet::iter() count = %d (expect 2)\n", hsc);
        assert(hsc == 2);
        hsi.methods->hicc_destroy(hsi);

        hs.methods->hicc_destroy(hs);
    }

    /* ===== HashSet<String> ===== */
    puts("\n--- HashSet<String> ---");
    {
        struct AbiClass_HashSet_String hss = fn->new_hashset_string();
        assert(hss.methods->is_empty(&hss));

        struct AbiClass_String hk1 = fn->new_string_empty();
        hk1.methods->push_cstr(&hk1, (const int8_t *)"abc");
        bool hins = hss.methods->insert(&hss, hk1);
        assert(hins);
        printf("HashSet<String>::insert() = %s\n", hins ? "true" : "false");

        struct AbiClass_set_Iter_string_String hsi = hss.methods->iter(&hss);
        int hsc = 0;
        while (true) {
            struct AbiClass_option_Option_string_String he = hsi.methods->next(&hsi);
            if (he.methods->is_none(&he)) { he.methods->hicc_destroy(he); break; }
            hsc++;
            he.methods->hicc_destroy(he);
        }
        printf("HashSet<String>::iter() count = %d (expect 1)\n", hsc);
        assert(hsc == 1);
        hsi.methods->hicc_destroy(hsi);

        hss.methods->hicc_destroy(hss);
    }

    /* ===== BTreeMap<i32, i32> into_iter ===== */
    puts("\n--- BTreeMap<i32, i32> into_iter ---");
    {
        struct AbiClass_BTreeMap_i32_i32 bm2 = fn->new_btreemap_i32_i32();
        bm2.methods->insert(&bm2, 1, 10);
        bm2.methods->insert(&bm2, 2, 20);

        struct AbiClass_IntoIter_i32_i32 into_it = bm2.methods->into_iter(bm2);
        /* bm2 is consumed by into_iter — DO NOT destroy bm2 */
        int itc = 0;
        while (true) {
            struct AbiClass_option_Option_i32_i32 ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            struct AbiClass_i32_i32 tup = ent.methods->unwrap(ent);
            /* ent consumed by unwrap */
            const int32_t *k_ptr = tup.methods->field_0(&tup);
            const int32_t *v_ptr = tup.methods->field_1(&tup);
            printf("  into_iter item: key=%d val=%d\n", *k_ptr, *v_ptr);
            tup.methods->hicc_destroy(tup);
            itc++;
        }
        printf("BTreeMap::into_iter() count = %d (expect 2)\n", itc);
        assert(itc == 2);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== BTreeSet<i32> into_iter ===== */
    puts("\n--- BTreeSet<i32> into_iter ---");
    {
        struct AbiClass_BTreeSet_i32 bs2 = fn->new_btreeset_i32();
        bs2.methods->insert(&bs2, 10);
        bs2.methods->insert(&bs2, 20);

        struct AbiClass_IntoIter_i32 into_it = bs2.methods->into_iter(bs2);
        /* bs2 consumed by into_iter — DO NOT destroy bs2 */
        int itc = 0;
        while (true) {
            struct AbiClass_Option_i32 ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            int32_t val = ent.methods->unwrap(ent);
            /* ent consumed by unwrap */
            printf("  into_iter item: %d\n", val);
            itc++;
        }
        printf("BTreeSet::into_iter() count = %d (expect 2)\n", itc);
        assert(itc == 2);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== HashMap<i32, i32> into_iter ===== */
    puts("\n--- HashMap<i32, i32> into_iter ---");
    {
        struct AbiClass_HashMap_i32_i32 hm2 = fn->new_hashmap_i32_i32();
        hm2.methods->insert(&hm2, 1, 100);
        hm2.methods->insert(&hm2, 2, 200);

        struct AbiClass_map_IntoIter_i32_i32 into_it = hm2.methods->into_iter(hm2);
        /* hm2 consumed by into_iter — DO NOT destroy hm2 */
        int itc = 0;
        while (true) {
            struct AbiClass_option_Option_i32_i32 ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            struct AbiClass_i32_i32 tup = ent.methods->unwrap(ent);
            const int32_t *k_ptr = tup.methods->field_0(&tup);
            const int32_t *v_ptr = tup.methods->field_1(&tup);
            printf("  into_iter item: key=%d val=%d\n", *k_ptr, *v_ptr);
            tup.methods->hicc_destroy(tup);
            itc++;
        }
        printf("HashMap::into_iter() count = %d (expect 2)\n", itc);
        assert(itc == 2);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== HashSet<i32> into_iter ===== */
    puts("\n--- HashSet<i32> into_iter ---");
    {
        struct AbiClass_HashSet_i32 hs2 = fn->new_hashset_i32();
        hs2.methods->insert(&hs2, 42);
        hs2.methods->insert(&hs2, 43);

        struct AbiClass_set_IntoIter_i32 into_it = hs2.methods->into_iter(hs2);
        /* hs2 consumed by into_iter — DO NOT destroy hs2 */
        int itc = 0;
        while (true) {
            struct AbiClass_Option_i32 ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            int32_t val = ent.methods->unwrap(ent);
            printf("  into_iter item: %d\n", val);
            itc++;
        }
        printf("HashSet::into_iter() count = %d (expect 2)\n", itc);
        assert(itc == 2);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== BTreeMap<String, String> into_iter ===== */
    puts("\n--- BTreeMap<String, String> into_iter ---");
    {
        struct AbiClass_BTreeMap_String_String bms2 = fn->new_btreemap_string_string();
        struct AbiClass_String k1 = fn->new_string();
        struct AbiClass_String v1 = fn->new_string();
        struct AbiClass_Option_String ins1 = bms2.methods->insert(&bms2, k1, v1);
        /* k1, v1 consumed by insert */
        ins1.methods->hicc_destroy(ins1);

        struct AbiClass_String k2 = fn->new_string_empty();
        k2.methods->push_cstr(&k2, (const int8_t *)"key2");
        struct AbiClass_String v2 = fn->new_string_empty();
        v2.methods->push_cstr(&v2, (const int8_t *)"val2");
        struct AbiClass_Option_String ins2 = bms2.methods->insert(&bms2, k2, v2);
        /* k2, v2 consumed by insert */
        ins2.methods->hicc_destroy(ins2);

        struct AbiClass_IntoIter_String_String into_it = bms2.methods->into_iter(bms2);
        /* bms2 consumed by into_iter — DO NOT destroy bms2 */
        int itc = 0;
        while (true) {
            struct AbiClass_option_Option_string_String_string_String ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            struct AbiClass_string_String_string_String tup = ent.methods->unwrap(ent);
            /* ent consumed by unwrap */
            struct AbiClass_String k_s = tup.methods->take_0(tup);
            /* tup consumed by take_0 (take_1 would need tup, but take_0 already consumed tup's self) */
            uintptr_t klen = k_s.methods->len(&k_s);
            printf("  into_iter key len = %lu\n", (unsigned long)klen);
            k_s.methods->hicc_destroy(k_s);
            itc++;
        }
        printf("BTreeMap<String>::into_iter() count = %d (expect 2)\n", itc);
        assert(itc == 2);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== BTreeSet<String> into_iter ===== */
    puts("\n--- BTreeSet<String> into_iter ---");
    {
        struct AbiClass_BTreeSet_String bss2 = fn->new_btreeset_string();
        struct AbiClass_String sk1 = fn->new_string_empty();
        sk1.methods->push_cstr(&sk1, (const int8_t *)"abc");
        bss2.methods->insert(&bss2, sk1);
        /* sk1 consumed by insert */

        struct AbiClass_IntoIter_String into_it = bss2.methods->into_iter(bss2);
        /* bss2 consumed by into_iter — DO NOT destroy bss2 */
        int itc = 0;
        while (true) {
            struct AbiClass_Option_String ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            struct AbiClass_String val = ent.methods->unwrap(ent);
            /* ent consumed by unwrap */
            uintptr_t vlen = val.methods->len(&val);
            printf("  into_iter val len = %lu (expect 3)\n", (unsigned long)vlen);
            assert(vlen == 3);
            val.methods->hicc_destroy(val);
            itc++;
        }
        printf("BTreeSet<String>::into_iter() count = %d (expect 1)\n", itc);
        assert(itc == 1);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== HashMap<String, String> into_iter ===== */
    puts("\n--- HashMap<String, String> into_iter ---");
    {
        struct AbiClass_HashMap_String_String hms2 = fn->new_hashmap_string_string();
        struct AbiClass_String hk = fn->new_string_empty();
        hk.methods->push_cstr(&hk, (const int8_t *)"k1");
        struct AbiClass_String hv = fn->new_string_empty();
        hv.methods->push_cstr(&hv, (const int8_t *)"v1");
        struct AbiClass_Option_String hins = hms2.methods->insert(&hms2, hk, hv);
        /* hk, hv consumed by insert */
        hins.methods->hicc_destroy(hins);

        struct AbiClass_map_IntoIter_string_String_string_String into_it = hms2.methods->into_iter(hms2);
        /* hms2 consumed by into_iter — DO NOT destroy hms2 */
        int itc = 0;
        while (true) {
            struct AbiClass_option_Option_string_String_string_String ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            struct AbiClass_string_String_string_String tup = ent.methods->unwrap(ent);
            /* ent consumed by unwrap */
            struct AbiClass_String k_s = tup.methods->take_0(tup);
            /* tup consumed by take_0 */
            uintptr_t klen = k_s.methods->len(&k_s);
            printf("  into_iter key len = %lu\n", (unsigned long)klen);
            k_s.methods->hicc_destroy(k_s);
            itc++;
        }
        printf("HashMap<String>::into_iter() count = %d (expect 1)\n", itc);
        assert(itc == 1);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== HashSet<String> into_iter ===== */
    puts("\n--- HashSet<String> into_iter ---");
    {
        struct AbiClass_HashSet_String hss2 = fn->new_hashset_string();
        struct AbiClass_String hk1 = fn->new_string_empty();
        hk1.methods->push_cstr(&hk1, (const int8_t *)"abc");
        hss2.methods->insert(&hss2, hk1);
        /* hk1 consumed by insert */

        struct AbiClass_set_IntoIter_string_String into_it = hss2.methods->into_iter(hss2);
        /* hss2 consumed by into_iter — DO NOT destroy hss2 */
        int itc = 0;
        while (true) {
            struct AbiClass_Option_String ent = into_it.methods->next(&into_it);
            if (ent.methods->is_none(&ent)) { ent.methods->hicc_destroy(ent); break; }
            struct AbiClass_String val = ent.methods->unwrap(ent);
            /* ent consumed by unwrap */
            uintptr_t vlen = val.methods->len(&val);
            printf("  into_iter val len = %lu (expect 3)\n", (unsigned long)vlen);
            assert(vlen == 3);
            val.methods->hicc_destroy(val);
            itc++;
        }
        printf("HashSet<String>::into_iter() count = %d (expect 1)\n", itc);
        assert(itc == 1);
        into_it.methods->hicc_destroy(into_it);
    }

    /* ===== NonNull<String> — behaves like &mut String ===== */
    puts("\n--- NonNull<String> ---");
    {
        struct AbiClass_String ns = fn->nonnull_string();
        uintptr_t nlen = ns.methods->len(&ns);
        printf("NonNull<String>::len() = %lu (expect 5)\n", (unsigned long)nlen);
        assert(nlen == 5);

        struct AbiClass_str nsr = ns.methods->as_str(&ns);
        assert(nsr.methods->len(&nsr) == 5);
        nsr.methods->hicc_destroy(nsr);

        ns.methods->push_cstr(&ns, (const int8_t *)" world");
        printf("NonNull<String>::push_cstr()->len() = %lu (expect 11)\n", (unsigned long)ns.methods->len(&ns));
        assert(ns.methods->len(&ns) == 11);

        /* NonNull<String> is IsMut (like &mut String), destroy = mem::forget (no data free) */
        ns.methods->hicc_destroy(ns);
    }

    /* ===== Cell<i32> ===== */
    puts("\n--- Cell<i32> ---");
    {
        struct AbiClass_Cell_i32 cell = fn->new_cell_i32();
        int32_t rpl = cell.methods->replace(&cell, 100);
        printf("Cell<i32>::replace() = %d (expect 42)\n", rpl);
        assert(rpl == 42);

        cell.methods->set(&cell, 200);

        /* Cell<i32>::as_ptr() → read/write through raw pointer */
        int32_t *cptr = cell.methods->as_ptr(&cell);
        printf("Cell<i32>::as_ptr() read = %d (expect 200)\n", *cptr);
        assert(*cptr == 200);
        *cptr = 300;
        int32_t rpl2 = cell.methods->replace(&cell, 0);
        printf("Cell<i32>::replace() after as_ptr write = %d (expect 300)\n", rpl2);
        assert(rpl2 == 300);

        int32_t val = cell.methods->into_inner(cell);
        printf("Cell<i32>::into_inner() = %d (expect 0)\n", val);
        assert(val == 0);
    }

    /* ===== RefCell<i32> ===== */
    puts("\n--- RefCell<i32> ---");
    {
        struct AbiClass_RefCell_i32 rc = fn->new_refcell_i32();
        int32_t rpl = rc.methods->replace(&rc, 100);
        printf("RefCell<i32>::replace() = %d (expect 42)\n", rpl);
        assert(rpl == 42);

        int32_t *gm = rc.methods->get_mut(&rc);
        *gm = 200;

        /* RefCell<i32>::as_ptr() → read/write through raw pointer */
        int32_t *rptr = rc.methods->as_ptr(&rc);
        printf("RefCell<i32>::as_ptr() read = %d (expect 200)\n", *rptr);
        assert(*rptr == 200);
        *rptr = 300;
        int32_t rpl2 = rc.methods->replace(&rc, 0);
        printf("RefCell<i32>::replace() after as_ptr write = %d (expect 300)\n", rpl2);
        assert(rpl2 == 300);

        int32_t val = rc.methods->into_inner(rc);
        printf("RefCell<i32>::into_inner() = %d (expect 0)\n", val);
        assert(val == 0);
    }

    /* ===== OnceLock<i32> ===== */
    puts("\n--- OnceLock<i32> ---");
    {
        struct AbiClass_OnceLock_i32 lock = fn->new_oncelock_i32();

        /* get before set = None */
        struct AbiClass_option_Option_i32 before = lock.methods->get(&lock);
        assert(before.methods->is_none(&before));
        before.methods->hicc_destroy(before);

        /* set(42) should succeed */
        struct AbiClass_Result_i32 set1 = lock.methods->set(&lock, 42);
        assert(set1.methods->is_ok(&set1));
        set1.methods->hicc_destroy(set1);

        /* set(99) should fail, returning Err(99) */
        struct AbiClass_Result_i32 set2 = lock.methods->set(&lock, 99);
        assert(set2.methods->is_err(&set2));
        int32_t err_val = set2.methods->err(set2);
        printf("OnceLock<i32>::double_set err = %d (expect 99)\n", err_val);
        assert(err_val == 99);

        /* get should return Some(&42) */
        struct AbiClass_option_Option_i32 get1 = lock.methods->get(&lock);
        assert(!get1.methods->is_none(&get1));
        {
            const int32_t *const *val_ptr = get1.methods->as_ref(&get1);
            const int32_t *gvval = *val_ptr;
            printf("OnceLock<i32>::get() = %d (expect 42)\n", *gvval);
            assert(*gvval == 42);
        }
        get1.methods->hicc_destroy(get1);

        /* into_inner should return Some(42) */
        struct AbiClass_Option_i32 inner = lock.methods->into_inner(lock);
        assert(!inner.methods->is_none(&inner));
        int32_t ival = inner.methods->unwrap(inner);
        printf("OnceLock<i32>::into_inner() = %d (expect 42)\n", ival);
        assert(ival == 42);
    }

    /* ===== Mutex<i32> ===== */
    puts("\n--- Mutex<i32> ---");
    {
        struct AbiClass_Mutex_i32 mtx = fn->new_mutex_i32();
        assert(!mtx.methods->is_poisoned(&mtx));

        /* lock and get */
        struct AbiClass_MutexGuard_i32 guard = mtx.methods->lock(&mtx);
        const int32_t *gv = guard.methods->get(&guard);
        printf("Mutex<i32>::lock()->get() = %d (expect 42)\n", *gv);
        assert(*gv == 42);

        /* get_mut through guard */
        int32_t *gm = guard.methods->get_mut(&guard);
        *gm = 99;
        const int32_t *gv2 = guard.methods->get(&guard);
        printf("Mutex<i32>::lock()->get_mut()=99, get() = %d (expect 99)\n", *gv2);
        assert(*gv2 == 99);
        guard.methods->hicc_destroy(guard);

        /* try_lock should succeed (no longer locked) */
        struct AbiClass_Option_MutexGuard_i32 try_opt = mtx.methods->try_lock(&mtx);
        assert(!try_opt.methods->is_none(&try_opt));
        struct AbiClass_MutexGuard_i32 try_g = try_opt.methods->unwrap(try_opt);
        const int32_t *try_v = try_g.methods->get(&try_g);
        printf("Mutex<i32>::try_lock()->get() = %d (expect 99)\n", *try_v);
        assert(*try_v == 99);
        try_g.methods->hicc_destroy(try_g);

        /* get_mut on mutex itself */
        int32_t *mm = mtx.methods->get_mut(&mtx);
        *mm = 100;

        /* into_inner */
        int32_t inner = mtx.methods->into_inner(mtx);
        printf("Mutex<i32>::into_inner() = %d (expect 100)\n", inner);
        assert(inner == 100);
    }

    /* ===== RwLock<i32> ===== */
    puts("\n--- RwLock<i32> ---");
    {
        struct AbiClass_RwLock_i32 rwl = fn->new_rwlock_i32();
        assert(!rwl.methods->is_poisoned(&rwl));

        /* read lock */
        struct AbiClass_RwLockReadGuard_i32 rguard = rwl.methods->read(&rwl);
        const int32_t *rv = rguard.methods->get(&rguard);
        printf("RwLock<i32>::read()->get() = %d (expect 42)\n", *rv);
        assert(*rv == 42);
        rguard.methods->hicc_destroy(rguard);

        /* write lock and modify */
        struct AbiClass_RwLockWriteGuard_i32 wguard = rwl.methods->write(&rwl);
        int32_t *wgm = wguard.methods->get_mut(&wguard);
        *wgm = 77;
        const int32_t *wv = wguard.methods->get(&wguard);
        printf("RwLock<i32>::write()->get_mut()=77, get() = %d (expect 77)\n", *wv);
        assert(*wv == 77);
        wguard.methods->hicc_destroy(wguard);

        /* get_mut on rwlock itself */
        int32_t *rm = rwl.methods->get_mut(&rwl);
        *rm = 88;

        /* into_inner */
        int32_t rinner = rwl.methods->into_inner(rwl);
        printf("RwLock<i32>::into_inner() = %d (expect 88)\n", rinner);
        assert(rinner == 88);
    }

    puts("\nrust-std example passed!");
    return 0;
}