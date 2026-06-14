import ctypes

from hicc_rust_std import *


def test_option_i32():
    print("\n--- Option<i32> ---")
    opt = new_option_i32()
    is_none = opt.is_none()
    print(f"Option<i32>::is_none() = {str(is_none).lower()} (expect false)")
    assert not is_none

    sz = opt._inner.methods[0].hicc_size_of()
    print(f"Option<i32>::size_of() = {sz}")

    p = opt.as_ref()
    print(f"Option<i32>::as_ref() = {p} (expect 42)")
    assert p == 42

    val = opt.unwrap()
    print(f"Option<i32>::unwrap() = {val} (expect 42)")
    assert val == 42


def test_option_i32_none():
    print("\n--- Option<i32> None ---")
    none = new_option_none_i32()
    is_none = none.is_none()
    print(f"Option<i32>::is_none() = {str(is_none).lower()} (expect true)")
    assert is_none
    none.destroy()


def test_option_string():
    print("\n--- Option<String> ---")
    opts = new_option_string()
    is_none = opts.is_none()
    assert not is_none

    inner = opts.as_ref()
    slen = inner.len()
    print(f"Option<String>::as_ref()->len() = {slen} (expect 5)")
    assert slen == 5
    inner.destroy()

    taken = opts.take()
    taken_none = taken.is_none()
    assert not taken_none
    unwrapped = taken.unwrap()
    ulen = unwrapped.len()
    print(f"Option<String>::take()->unwrap()->len() = {ulen} (expect 5)")
    assert ulen == 5
    unwrapped.destroy()

    orig_none = opts.is_none()
    print(f"After take, original is_none = {str(orig_none).lower()} (expect true)")
    assert orig_none
    opts.destroy()


def test_result_ok():
    print("\n--- Result<i32, bool> Ok ---")
    ok = new_result_ok_i32()
    assert ok.is_ok()
    assert not ok.is_err()
    val = ok.ok()
    print(f"Result::ok() = {val} (expect 100)")
    assert val == 100


def test_result_err():
    print("\n--- Result<i32, bool> Err ---")
    err = new_result_err_bool()
    assert err.is_err()
    assert not err.is_ok()
    val = err.err()
    print(f"Result::err() = {str(val).lower()} (expect false)")
    assert not val


def test_array_option_i32():
    print("\n--- Array<Option<i32>, 3> ---")
    arr = new_array_option_i32()
    alen = arr.len()
    print(f"Array::len() = {alen} (expect 3)")
    assert alen == 3

    item0 = arr.get(0)
    item0_none = item0.is_none()
    assert not item0_none
    v0 = item0.as_ref()
    print(f"Array::get(0)->as_ref() = {v0} (expect 10)")
    assert v0 == 10
    item0.destroy()

    _tmp_opt = new_option_i32()
    arr.set(0, _tmp_opt)
    new0 = arr.get(0)
    nv0 = new0.as_ref()
    print(f"Array::set(0,Some(42))::get(0)->as_ref() = {nv0} (expect 42)")
    assert nv0 == 42
    new0.destroy()

    item1 = arr.get(1)
    assert item1.is_none()
    item1.destroy()

    arr.destroy()


def test_tuple_i32_i32():
    print("\n--- Tuple (i32, i32) ---")
    t = new_tuple_i32_i32()
    f0 = t.field_0()
    f1 = t.field_1()
    print(f"Tuple::field_0() = {f0}, field_1() = {f1} (expect 1, 2)")
    assert f0 == 1 and f1 == 2

    v0 = t.take_0()
    print(f"Tuple::take_0() = {v0} (expect 1)")
    assert v0 == 1


def test_tuple_i32_string():
    print("\n--- Tuple (i32, String) ---")
    ts = new_tuple_i32_string()
    tf0 = ts.field_0()
    print(f"Tuple::field_0() = {tf0} (expect 99)")
    assert tf0 == 99

    tf1 = ts.field_1()
    tflen = tf1.len()
    print(f"Tuple::field_1()->len() = {tflen} (expect 5)")
    assert tflen == 5
    tf1.destroy()

    tv0 = ts.take_0()
    print(f"Tuple::take_0() = {tv0} (expect 99)")
    assert tv0 == 99


def test_tuple3_to_6():
    print("\n--- Tuple3..6 ---")
    t3 = new_tuple_i32_i32_i32()
    assert t3.field_0() == 1
    assert t3.field_1() == 2
    assert t3.field_2() == 3
    print("Tuple3::field_0/1/2 = 1/2/3 OK")
    assert t3.take_0() == 1

    t4 = new_tuple_i32_i32_i32_i32()
    assert t4.field_0() == 1
    assert t4.field_1() == 2
    assert t4.field_2() == 3
    assert t4.field_3() == 4
    print("Tuple4::field_0/1/2/3 = 1/2/3/4 OK")
    assert t4.take_0() == 1

    t5 = new_tuple_i32_i32_i32_i32_i32()
    assert t5.field_0() == 1
    assert t5.field_4() == 5
    print("Tuple5::field_0/4 = 1/5 OK")
    assert t5.take_0() == 1

    t6 = new_tuple_i32_i32_i32_i32_i32_i32()
    assert t6.field_0() == 1
    assert t6.field_5() == 6
    print("Tuple6::field_0/5 = 1/6 OK")
    assert t6.take_0() == 1


def test_str():
    print("\n--- &str ---")
    s = new_str()
    slen = s.len()
    print(f"str::len() = {slen} (expect 5)")
    assert slen == 5

    ch = s.get(0)
    print(f"str::get(0) = '{chr(ch)}' (expect 'h')")
    assert ch == ord('h')

    s.destroy()


def test_vec_i32():
    print("\n--- Vec<i32> ---")
    v = new_vec_i32()
    assert v.len() == 0

    v.push(10)
    v.push(20)
    v.push(30)
    print(f"Vec<i32>::len() after 3 pushes = {v.len()} (expect 3)")
    assert v.len() == 3

    g0 = v.get(0)
    print(f"Vec<i32>::get(0) = {g0} (expect 10)")
    assert g0 == 10

    g1 = v.get(1)
    print(f"Vec<i32>::get(1) = {g1} (expect 20)")
    assert g1 == 20

    popped = v.pop()
    pval = popped.unwrap()
    print(f"Vec<i32>::pop() = {pval} (expect 30)")
    assert pval == 30

    slice = v.as_slice()
    print(f"Vec<i32>::as_slice()->len() = {slice.len()} (expect 2)")
    assert slice.len() == 2
    slice.destroy()

    v.destroy()


def test_vec_string():
    print("\n--- Vec<String> ---")
    vs = new_vec_string()
    assert vs.len() == 0

    s1 = new_string()
    vs.push(s1)
    print(f"Vec<String>::len() after push = {vs.len()} (expect 1)")
    assert vs.len() == 1

    g0s = vs.get(0)
    print(f"Vec<String>::get(0)->len() = {g0s.len()} (expect 5)")
    assert g0s.len() == 5
    g0s.destroy()

    sls = vs.as_slice()
    print(f"Vec<String>::as_slice()->len() = {sls.len()} (expect 1)")
    assert sls.len() == 1
    sls.destroy()

    ps = vs.pop()
    pv = ps.unwrap()
    print(f"Vec<String>::pop()->unwrap()->len() = {pv.len()} (expect 5)")
    assert pv.len() == 5
    pv.destroy()

    vs.destroy()


def test_string():
    print("\n--- String ---")
    st = new_string()
    assert st.len() == 5

    sr = st.as_str()
    assert sr.len() == 5
    sr.destroy()

    sb = st.as_bytes()
    assert sb.len() == 5
    sb.destroy()

    # push_str: value-passed &str arg is consumed by Rust side
    push_arg1 = new_str()
    st.push_str(push_arg1)
    # push_arg1 is consumed — wrapper already nullified its _inner
    print(f"String::push_str()->len() = {st.len()} (expect 10)")
    assert st.len() == 10

    st.push_cstr(b' ')
    st.push_cstr(b'w')
    st.push_cstr(b'o')
    st.push_cstr(b'r')
    st.push_cstr(b'l')
    st.push_cstr(b'd')
    print(f"String::push_cstr()->len() = {st.len()} (expect 16)")
    assert st.len() == 16

    # insert_str: value-passed &str arg is consumed by Rust side
    push_arg2 = new_str()
    st.insert_str(5, push_arg2)
    # push_arg2 is consumed — wrapper already nullified its _inner
    st.insert_cstr(10, b' ')
    st.insert_cstr(11, b'b')
    st.insert_cstr(12, b'e')
    st.insert_cstr(13, b'a')
    st.insert_cstr(14, b'u')
    st.insert_cstr(15, b't')
    st.insert_cstr(16, b'i')
    st.insert_cstr(17, b'f')
    st.insert_cstr(18, b'u')
    st.insert_cstr(19, b'l')
    print(f"String after insert_str+insert_cstr len = {st.len()}")

    st.destroy()


def test_string_empty():
    print("\n--- String (empty) ---")
    se = new_string_empty()
    assert se.len() == 0

    se.push_cstr(b'h')
    se.push_cstr(b'e')
    se.push_cstr(b'l')
    se.push_cstr(b'l')
    se.push_cstr(b'o')
    print(f"empty String::push_cstr()->len() = {se.len()} (expect 5)")
    assert se.len() == 5

    se.insert_cstr(5, b' ')
    se.insert_cstr(6, b'w')
    se.insert_cstr(7, b'o')
    se.insert_cstr(8, b'r')
    se.insert_cstr(9, b'l')
    se.insert_cstr(10, b'd')
    print(f"String::insert_cstr(5,\" world\")->len() = {se.len()} (expect 11)")
    assert se.len() == 11

    se.destroy()


def test_box_i32():
    print("\n--- Box<i32> ---")
    b = new_box_i32()
    bg = b.get()
    print(f"Box::get() = {bg} (expect 42)")
    assert bg == 42

    # Write through raw get_mut pointer (matches C: *bm = 100)
    ptr = b._inner.methods[0].get_mut(ctypes.byref(b._inner))
    ptr[0] = 100
    bg2 = b.get()
    print(f"Box::get_mut()=100, get() = {bg2} (expect 100)")
    assert bg2 == 100

    b.destroy()


def test_box_string():
    print("\n--- Box<String> ---")
    # Box<String> 的 Abi 类型是 AbiClass_String，可直接调用 String 的方法
    bs = new_box_string()
    bslen = bs.len()
    print(f"Box<String>::len() = {bslen} (expect 5)")
    assert bslen == 5

    bsr = bs.as_str()
    assert bsr.len() == 5
    bsr.destroy()

    bs.push_cstr(b' ')
    bs.push_cstr(b'w')
    bs.push_cstr(b'o')
    bs.push_cstr(b'r')
    bs.push_cstr(b'l')
    bs.push_cstr(b'd')
    print(f"Box<String>::push_cstr()->len() = {bs.len()} (expect 11)")
    assert bs.len() == 11

    bs.destroy()


def test_rc_i32():
    print("\n--- Rc<i32> ---")
    rc = new_rc_i32()
    rvg = rc.get()
    print(f"Rc::get() = {rvg} (expect 42)")
    assert rvg == 42
    rc.destroy()


def test_arc_i32():
    print("\n--- Arc<i32> ---")
    arc = new_arc_i32()
    avg = arc.get()
    print(f"Arc::get() = {avg} (expect 42)")
    assert avg == 42
    arc.destroy()


def test_btreemap_i32_i32():
    print("\n--- BTreeMap<i32, i32> ---")
    bm = new_btreemap_i32_i32()
    assert bm.is_empty()
    assert bm.len() == 0

    ins_ret = bm.insert(1, 100)
    ins_none = ins_ret.is_none()
    assert ins_none
    ins_ret.destroy()

    bm.insert(2, 200)
    bm.insert(3, 300)
    print(f"BTreeMap::len() after 3 inserts = {bm.len()} (expect 3)")
    assert bm.len() == 3

    assert bm.contains_key(1)
    assert not bm.contains_key(99)

    gv = bm.get(1)
    gvval = gv.as_ref()
    print(f"BTreeMap::get(1)->as_ref() = {gvval} (expect 100)")
    assert gvval == 100
    gv.destroy()

    rm_ret = bm.remove(1)
    rm_val = rm_ret.unwrap()
    print(f"BTreeMap::remove(1) = {rm_val} (expect 100)")
    assert rm_val == 100
    assert bm.len() == 2

    iter = bm.iter()
    count = 0
    while True:
        entry = iter.next()
        if entry.is_none():
            entry.destroy()
            break
        count += 1
        entry.destroy()
    print(f"BTreeMap::iter() count = {count} (expect 2)")
    assert count == 2
    iter.destroy()

    bm.destroy()


def test_btreemap_string_string():
    print("\n--- BTreeMap<String, String> ---")
    bms = new_btreemap_string_string()
    assert bms.is_empty()

    # insert consumes k1, v1 by value
    k1 = new_string()
    v1 = new_string()
    ins1 = bms.insert(k1, v1)
    assert ins1.is_none()
    ins1.destroy()

    k2 = new_string_empty()
    for c in "key2":
        k2.push_cstr(c.encode())
    v2 = new_string_empty()
    for c in "val2":
        v2.push_cstr(c.encode())
    ins2 = bms.insert(k2, v2)
    assert ins2.is_none()
    ins2.destroy()

    print(f"BTreeMap<String>::len() = {bms.len()} (expect 2)")
    assert bms.len() == 2

    iters = bms.iter()
    scount = 0
    while True:
        ent = iters.next()
        if ent.is_none():
            ent.destroy()
            break
        scount += 1
        ent.destroy()
    print(f"BTreeMap<String>::iter() count = {scount} (expect 2)")
    assert scount == 2
    iters.destroy()

    bms.destroy()


def test_btreeset_i32():
    print("\n--- BTreeSet<i32> ---")
    bs = new_btreeset_i32()
    assert bs.is_empty()

    inserted = bs.insert(10)
    print(f"BTreeSet::insert(10) = {str(inserted).lower()} (expect true)")
    assert inserted
    bs.insert(20)
    bs.insert(30)
    assert bs.len() == 3

    assert bs.contains(10)
    assert not bs.contains(99)

    removed = bs.remove(10)
    print(f"BTreeSet::remove(10) = {str(removed).lower()} (expect true)")
    assert removed
    assert bs.len() == 2

    bsiter = bs.iter()
    bscount = 0
    while True:
        e = bsiter.next()
        if e.is_none():
            e.destroy()
            break
        bscount += 1
        e.destroy()
    print(f"BTreeSet::iter() count = {bscount} (expect 2)")
    assert bscount == 2
    bsiter.destroy()

    bs.destroy()


def test_btreeset_string():
    print("\n--- BTreeSet<String> ---")
    bss = new_btreeset_string()
    assert bss.is_empty()

    # insert consumes sk1 by value
    sk1 = new_string_empty()
    for c in "abc":
        sk1.push_cstr(c.encode())
    ins = bss.insert(sk1)
    assert ins
    print(f"BTreeSet<String>::insert() = {str(ins).lower()}")

    bsi = bss.iter()
    bsc = 0
    while True:
        e = bsi.next()
        if e.is_none():
            e.destroy()
            break
        bsc += 1
        e.destroy()
    print(f"BTreeSet<String>::iter() count = {bsc} (expect 1)")
    assert bsc == 1
    bsi.destroy()

    bss.destroy()


def test_hashmap_i32_i32():
    print("\n--- HashMap<i32, i32> ---")
    hm = new_hashmap_i32_i32()
    assert hm.is_empty()

    hm.insert(1, 100)
    hm.insert(2, 200)
    print(f"HashMap::len() = {hm.len()} (expect 2)")
    assert hm.len() == 2

    assert hm.contains_key(1)

    hgv = hm.get(1)
    hgval = hgv.as_ref()
    print(f"HashMap::get(1) = {hgval} (expect 100)")
    assert hgval == 100
    hgv.destroy()

    hrm = hm.remove(1)
    hrmv = hrm.unwrap()
    print(f"HashMap::remove(1) = {hrmv} (expect 100)")
    assert hrmv == 100

    hiter = hm.iter()
    hc = 0
    while True:
        he = hiter.next()
        if he.is_none():
            he.destroy()
            break
        hc += 1
        he.destroy()
    print(f"HashMap::iter() count = {hc} (expect 1)")
    assert hc == 1
    hiter.destroy()

    hm.destroy()


def test_hashmap_string_string():
    print("\n--- HashMap<String, String> ---")
    hms = new_hashmap_string_string()
    assert hms.is_empty()

    # insert consumes hk, hv by value
    hk = new_string_empty()
    for c in "k1":
        hk.push_cstr(c.encode())
    hv = new_string_empty()
    for c in "v1":
        hv.push_cstr(c.encode())
    hins = hms.insert(hk, hv)
    assert hins.is_none()
    hins.destroy()

    print(f"HashMap<String>::len() = {hms.len()} (expect 1)")
    assert hms.len() == 1

    hsi = hms.iter()
    hsc = 0
    while True:
        he = hsi.next()
        if he.is_none():
            he.destroy()
            break
        hsc += 1
        he.destroy()
    print(f"HashMap<String>::iter() count = {hsc} (expect 1)")
    assert hsc == 1
    hsi.destroy()

    hms.destroy()


def test_hashset_i32():
    print("\n--- HashSet<i32> ---")
    hs = new_hashset_i32()
    assert hs.is_empty()

    hins = hs.insert(42)
    assert hins
    hs.insert(43)
    print(f"HashSet::len() = {hs.len()} (expect 2)")
    assert hs.len() == 2

    assert hs.contains(42)

    hsi = hs.iter()
    hsc = 0
    while True:
        he = hsi.next()
        if he.is_none():
            he.destroy()
            break
        hsc += 1
        he.destroy()
    print(f"HashSet::iter() count = {hsc} (expect 2)")
    assert hsc == 2
    hsi.destroy()

    hs.destroy()


def test_hashset_string():
    print("\n--- HashSet<String> ---")
    hss = new_hashset_string()
    assert hss.is_empty()

    # insert consumes hk1 by value
    hk1 = new_string_empty()
    for c in "abc":
        hk1.push_cstr(c.encode())
    hins = hss.insert(hk1)
    assert hins
    print(f"HashSet<String>::insert() = {str(hins).lower()}")

    hsi = hss.iter()
    hsc = 0
    while True:
        he = hsi.next()
        if he.is_none():
            he.destroy()
            break
        hsc += 1
        he.destroy()
    print(f"HashSet<String>::iter() count = {hsc} (expect 1)")
    assert hsc == 1
    hsi.destroy()

    hss.destroy()


def test_btreemap_i32_i32_into_iter():
    print("\n--- BTreeMap<i32, i32> into_iter ---")
    bm2 = new_btreemap_i32_i32()
    bm2.insert(1, 10)
    bm2.insert(2, 20)

    into_it = bm2.into_iter()
    # bm2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        tup = ent.unwrap()
        # ent consumed by unwrap
        k_ptr = tup.field_0()
        v_ptr = tup.field_1()
        print(f"  into_iter item: key={k_ptr} val={v_ptr}")
        tup.destroy()
        itc += 1
    print(f"BTreeMap::into_iter() count = {itc} (expect 2)")
    assert itc == 2
    into_it.destroy()


def test_btreeset_i32_into_iter():
    print("\n--- BTreeSet<i32> into_iter ---")
    bs2 = new_btreeset_i32()
    bs2.insert(10)
    bs2.insert(20)

    into_it = bs2.into_iter()
    # bs2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        val = ent.unwrap()
        # ent consumed by unwrap
        print(f"  into_iter item: {val}")
        itc += 1
    print(f"BTreeSet::into_iter() count = {itc} (expect 2)")
    assert itc == 2
    into_it.destroy()


def test_hashmap_i32_i32_into_iter():
    print("\n--- HashMap<i32, i32> into_iter ---")
    hm2 = new_hashmap_i32_i32()
    hm2.insert(1, 100)
    hm2.insert(2, 200)

    into_it = hm2.into_iter()
    # hm2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        tup = ent.unwrap()
        k_ptr = tup.field_0()
        v_ptr = tup.field_1()
        print(f"  into_iter item: key={k_ptr} val={v_ptr}")
        tup.destroy()
        itc += 1
    print(f"HashMap::into_iter() count = {itc} (expect 2)")
    assert itc == 2
    into_it.destroy()


def test_hashset_i32_into_iter():
    print("\n--- HashSet<i32> into_iter ---")
    hs2 = new_hashset_i32()
    hs2.insert(42)
    hs2.insert(43)

    into_it = hs2.into_iter()
    # hs2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        val = ent.unwrap()
        print(f"  into_iter item: {val}")
        itc += 1
    print(f"HashSet::into_iter() count = {itc} (expect 2)")
    assert itc == 2
    into_it.destroy()


def test_btreemap_string_string_into_iter():
    print("\n--- BTreeMap<String, String> into_iter ---")
    bms2 = new_btreemap_string_string()
    # insert consumes k1, v1 by value
    k1 = new_string()
    v1 = new_string()
    ins1 = bms2.insert(k1, v1)
    ins1.destroy()

    k2 = new_string_empty()
    for c in "key2":
        k2.push_cstr(c.encode())
    v2 = new_string_empty()
    for c in "val2":
        v2.push_cstr(c.encode())
    ins2 = bms2.insert(k2, v2)
    ins2.destroy()

    into_it = bms2.into_iter()
    # bms2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        tup = ent.unwrap()
        # ent consumed by unwrap
        k_s = tup.take_0()
        # tup consumed by take_0
        klen = k_s.len()
        print(f"  into_iter key len = {klen}")
        k_s.destroy()
        itc += 1
    print(f"BTreeMap<String>::into_iter() count = {itc} (expect 2)")
    assert itc == 2
    into_it.destroy()


def test_btreeset_string_into_iter():
    print("\n--- BTreeSet<String> into_iter ---")
    bss2 = new_btreeset_string()
    # insert consumes sk1 by value
    sk1 = new_string_empty()
    for c in "abc":
        sk1.push_cstr(c.encode())
    bss2.insert(sk1)

    into_it = bss2.into_iter()
    # bss2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        val = ent.unwrap()
        # ent consumed by unwrap
        vlen = val.len()
        print(f"  into_iter val len = {vlen} (expect 3)")
        assert vlen == 3
        val.destroy()
        itc += 1
    print(f"BTreeSet<String>::into_iter() count = {itc} (expect 1)")
    assert itc == 1
    into_it.destroy()


def test_hashmap_string_string_into_iter():
    print("\n--- HashMap<String, String> into_iter ---")
    hms2 = new_hashmap_string_string()
    # insert consumes hk, hv by value
    hk = new_string_empty()
    for c in "k1":
        hk.push_cstr(c.encode())
    hv = new_string_empty()
    for c in "v1":
        hv.push_cstr(c.encode())
    hins = hms2.insert(hk, hv)
    hins.destroy()

    into_it = hms2.into_iter()
    # hms2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        tup = ent.unwrap()
        # ent consumed by unwrap
        k_s = tup.take_0()
        # tup consumed by take_0
        klen = k_s.len()
        print(f"  into_iter key len = {klen}")
        k_s.destroy()
        itc += 1
    print(f"HashMap<String>::into_iter() count = {itc} (expect 1)")
    assert itc == 1
    into_it.destroy()


def test_hashset_string_into_iter():
    print("\n--- HashSet<String> into_iter ---")
    hss2 = new_hashset_string()
    # insert consumes hk1 by value
    hk1 = new_string_empty()
    for c in "abc":
        hk1.push_cstr(c.encode())
    hss2.insert(hk1)

    into_it = hss2.into_iter()
    # hss2 consumed by into_iter — DO NOT destroy
    itc = 0
    while True:
        ent = into_it.next()
        if ent.is_none():
            ent.destroy()
            break
        val = ent.unwrap()
        # ent consumed by unwrap
        vlen = val.len()
        print(f"  into_iter val len = {vlen} (expect 3)")
        assert vlen == 3
        val.destroy()
        itc += 1
    print(f"HashSet<String>::into_iter() count = {itc} (expect 1)")
    assert itc == 1
    into_it.destroy()


def test_nonnull_string():
    print("\n--- NonNull<String> ---")
    ns = nonnull_string()
    nlen = ns.len()
    print(f"NonNull<String>::len() = {nlen} (expect 5)")
    assert nlen == 5

    nsr = ns.as_str()
    assert nsr.len() == 5
    nsr.destroy()

    for c in " world":
        ns.push_cstr(c.encode())
    print(f"NonNull<String>::push_cstr()->len() = {ns.len()} (expect 11)")
    assert ns.len() == 11

    # NonNull<String> is IsMut, destroy = mem::forget
    ns.destroy()


def test_cell_i32():
    print("\n--- Cell<i32> ---")
    cell = new_cell_i32()
    rpl = cell.replace(100)
    print(f"Cell<i32>::replace() = {rpl} (expect 42)")
    assert rpl == 42

    cell.set(200)
    cptr = cell.as_ptr()
    print(f"Cell<i32>::as_ptr() read = {cptr} (expect 200)")
    assert cptr == 200
    cell.set(300)
    rpl2 = cell.replace(0)
    print(f"Cell<i32>::replace() after as_ptr write = {rpl2} (expect 300)")
    assert rpl2 == 300

    val = cell.into_inner()
    print(f"Cell<i32>::into_inner() = {val} (expect 0)")
    assert val == 0


def test_refcell_i32():
    print("\n--- RefCell<i32> ---")
    rcell = new_refcell_i32()
    rpl = rcell.replace(100)
    print(f"RefCell<i32>::replace() = {rpl} (expect 42)")
    assert rpl == 42

    # Write through raw get_mut pointer (matches C: *gm = 200)
    _gm = rcell._inner.methods[0].get_mut(ctypes.byref(rcell._inner))
    _gm[0] = 200

    # Read through raw as_ptr pointer (matches C: *rptr)
    _rptr = rcell._inner.methods[0].as_ptr(ctypes.byref(rcell._inner))
    print(f"RefCell<i32>::as_ptr() read = {_rptr[0]} (expect 200)")
    assert _rptr[0] == 200

    # Write through raw as_ptr pointer (matches C: *rptr = 300)
    _rptr[0] = 300
    rpl2 = rcell.replace(0)
    print(f"RefCell<i32>::replace() after as_ptr write = {rpl2} (expect 300)")
    assert rpl2 == 300

    val = rcell.into_inner()
    print(f"RefCell<i32>::into_inner() = {val} (expect 0)")
    assert val == 0


def test_oncelock_i32():
    print("\n--- OnceLock<i32> ---")
    lock = new_oncelock_i32()

    before = lock.get()
    assert before.is_none()
    before.destroy()

    set1 = lock.set(42)
    assert set1.is_ok()
    set1.destroy()

    set2 = lock.set(99)
    assert set2.is_err()
    err_val = set2.err()
    print(f"OnceLock<i32>::double_set err = {err_val} (expect 99)")
    assert err_val == 99

    get1 = lock.get()
    assert not get1.is_none()
    gvval = get1.as_ref()
    print(f"OnceLock<i32>::get() = {gvval} (expect 42)")
    assert gvval == 42
    get1.destroy()

    inner = lock.into_inner()
    assert not inner.is_none()
    ival = inner.unwrap()
    print(f"OnceLock<i32>::into_inner() = {ival} (expect 42)")
    assert ival == 42


def test_mutex_i32():
    print("\n--- Mutex<i32> ---")
    mtx = new_mutex_i32()
    assert not mtx.is_poisoned()

    guard = mtx.lock()
    gv = guard.get()
    print(f"Mutex<i32>::lock()->get() = {gv} (expect 42)")
    assert gv == 42

    # Write through raw get_mut pointer in guard (matches C: *gm = 99)
    _gm = guard._inner.methods[0].get_mut(ctypes.byref(guard._inner))
    _gm[0] = 99
    gv2 = guard.get()
    print(f"Mutex<i32>::lock()->get_mut()=99, get() = {gv2} (expect 99)")
    assert gv2 == 99
    guard.destroy()

    try_opt = mtx.try_lock()
    assert not try_opt.is_none()
    try_g = try_opt.unwrap()
    try_v = try_g.get()
    print(f"Mutex<i32>::try_lock()->get() = {try_v} (expect 99)")
    assert try_v == 99
    try_g.destroy()

    # Write through raw get_mut pointer on mutex (matches C: *mm = 100)
    _mm = mtx._inner.methods[0].get_mut(ctypes.byref(mtx._inner))
    _mm[0] = 100
    inner = mtx.into_inner()
    print(f"Mutex<i32>::into_inner() = {inner} (expect 100)")
    assert inner == 100


def test_rwlock_i32():
    print("\n--- RwLock<i32> ---")
    rwl = new_rwlock_i32()
    assert not rwl.is_poisoned()

    rguard = rwl.read()
    rv = rguard.get()
    print(f"RwLock<i32>::read()->get() = {rv} (expect 42)")
    assert rv == 42
    rguard.destroy()

    wguard = rwl.write()
    # Write through raw get_mut pointer in write guard (matches C: *wgm = 77)
    _wgm = wguard._inner.methods[0].get_mut(ctypes.byref(wguard._inner))
    _wgm[0] = 77
    wv = wguard.get()
    print(f"RwLock<i32>::write()->get_mut()=77, get() = {wv} (expect 77)")
    assert wv == 77
    wguard.destroy()

    # Write through raw get_mut pointer on rwlock (matches C: *rm = 88)
    _rm = rwl._inner.methods[0].get_mut(ctypes.byref(rwl._inner))
    _rm[0] = 88
    rinner = rwl.into_inner()
    print(f"RwLock<i32>::into_inner() = {rinner} (expect 88)")
    assert rinner == 88


def main():
    print("rust_std() at ???")

    test_option_i32()
    test_option_i32_none()
    test_option_string()
    test_result_ok()
    test_result_err()
    test_array_option_i32()
    test_tuple_i32_i32()
    test_tuple_i32_string()
    test_tuple3_to_6()
    test_str()
    test_vec_i32()
    test_vec_string()
    test_string()
    test_string_empty()
    test_box_i32()
    test_box_string()
    test_rc_i32()
    test_arc_i32()
    test_btreemap_i32_i32()
    test_btreemap_string_string()
    test_btreeset_i32()
    test_btreeset_string()
    test_hashmap_i32_i32()
    test_hashmap_string_string()
    test_hashset_i32()
    test_hashset_string()
    test_btreemap_i32_i32_into_iter()
    test_btreeset_i32_into_iter()
    test_hashmap_i32_i32_into_iter()
    test_hashset_i32_into_iter()
    test_btreemap_string_string_into_iter()
    test_btreeset_string_into_iter()
    test_hashmap_string_string_into_iter()
    test_hashset_string_into_iter()
    test_nonnull_string()
    test_cell_i32()
    test_refcell_i32()
    test_oncelock_i32()
    test_mutex_i32()
    test_rwlock_i32()

    print("\nrust-std example passed!")


if __name__ == "__main__":
    main()