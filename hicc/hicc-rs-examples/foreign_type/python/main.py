"""Python test for foreign_type — matches C output exactly."""
import ctypes

from hicc_foreign_type import *


def main():
    fn = foreign_type_c()

    print("=== Lib entry ===")
    print(f"foreign_type() at {ctypes.addressof(fn.contents):#x}")

    # ===== str methods =====
    print("\n--- str methods ---")
    s = new_str()

    s_len = s.len()
    print(f"s.len() = {s_len} (expect 5)")

    s_size = s.size_of()
    print(f"s.size_of() = {s_size}")

    first = s.get(0)
    print(f"s.get(0) = '{chr(first)}' (expect 'h')")

    s_ref = s.make_ref()
    s_ref_len = s_ref.len()
    print(f"s_ref.len() = {s_ref_len} (expect 5)")

    s_mut = s.make_ref_mut()
    s_mut_len = s_mut.len()
    print(f"s_mut.len() = {s_mut_len} (expect 5)")

    s_owned = s.make_unique()
    # s consumed by make_unique
    s_owned_len = s_owned.len()
    print(f"s_owned.len() = {s_owned_len} (expect 5)")
    s_owned.destroy()
    # get a fresh s
    s = new_str()

    # ===== Vec methods =====
    print("\n--- Vec methods ---")
    v = new_vec_string()

    v_size = v.size_of()
    print(f"v.size_of() = {v_size}")

    v_len = v.len()
    print(f"v.len() before push = {v_len} (expect 0)")

    v_ref = v.make_ref()
    v_ref_len = v_ref.len()
    print(f"v_ref.len() = {v_ref_len} (expect 0)")

    v_mut = v.make_ref_mut()
    v_mut_len = v_mut.len()
    print(f"v_mut.len() = {v_mut_len} (expect 0)")

    # ===== push =====
    print("\n--- push ---")
    result = push(v, s)
    # s consumed by push (value-passed AbiClass arg in factory function)
    print(f"fn->push(&v, s) -> result")

    v_len = v.len()
    print(f"v.len() after push = {v_len} (expect 1)")

    # ===== String methods =====
    print("\n--- String methods ---")

    result_size = result.size_of()
    print(f"result.size_of() = {result_size}")

    result_len = result.len()
    print(f"result.len() = {result_len} (expect 5)")

    result_str = result.as_str()
    result_str_len = result_str.len()
    print(f"result.as_str().len() = {result_str_len} (expect 5)")

    result_first = result_str.get(0)
    print(f"result.as_str()[0] = '{chr(result_first)}' (expect 'h')")
    result_str.destroy()

    # push_str: value-passed &str arg is consumed by Rust side
    s2 = new_str()
    result.push_str(s2)
    # s2 consumed — wrapper already nullified its _inner
    result_len = result.len()
    print(f"result.len() after push_str = {result_len} (expect 10)")

    # ===== String.async_len via wait =====
    print("\n--- String.async_len via wait ---")
    rt3 = new_runtime()
    future = result.async_len()
    async_len = future.wait(rt3)
    print(f"async_len() = {async_len} (expect 10)")
    assert async_len == 10
    rt3.destroy()

    result_ref = result.make_ref()
    result_ref_len = result_ref.len()
    print(f"result_ref.len() = {result_ref_len} (expect 10)")
    result_ref.destroy()

    result_owned = result.make_unique()
    # result consumed by make_unique
    result_owned_len = result_owned.len()
    print(f"result_owned.len() = {result_owned_len} (expect 10)")
    result_owned.destroy()

    # get fresh s2 and result
    s2 = new_str()
    result = push(v, s2)

    # ===== Write =====
    print("\n--- Write ---")
    v_fresh = new_vec_string()
    s3 = new_str()
    r3 = push(v_fresh, s3)
    # v_fresh consumed by write (value-passed AbiClass arg)
    v.write(v_fresh)
    v_len = v.len()
    print(f"v.len() after write = {v_len} (expect 1)")
    r3.destroy()

    # ===== Vec.push via methods =====
    print("\n--- Vec.push via methods ---")
    s4 = new_str()
    s4_string = push(v, s4)
    v_len = v.len()
    print(f"v.len() after fn->push = {v_len} (expect 2)")

    # Push s4_string again via method table — value-passed AbiClass arg
    v.push(s4_string)
    # s4_string consumed by push — wrapper nullified its _inner
    v_len = v.len()
    print(f"v.len() after methods->push = {v_len} (expect 3)")

    # ===== is_foreign() inline method verification =====
    print("\n--- is_foreign() ---")
    v_is_foreign = v.is_foreign()
    print(f"v.is_foreign() = {v_is_foreign} (expect True)")
    assert v_is_foreign
    result_is_foreign = result.is_foreign()
    print(f"result.is_foreign() = {result_is_foreign} (expect True)")
    assert result_is_foreign

    # ===== HashSet<Foreign<String>>: new_set_foreign_string + process_foreign_set =====
    print("\n--- HashSet<Foreign<String>> ---")
    hs = new_set_foreign_string()

    hs_len = hs.len()
    print(f"hs.len() after new_set_foreign_string = {hs_len} (expect 0)")
    assert hs_len == 0

    # Create a Foreign_String and insert it (uses existing v which is still alive)
    s5 = new_str()
    s5_fs = push(v, s5)
    inserted = hs.insert(s5_fs)
    # s5_fs consumed by insert
    print(f"hs.insert(s5_fs) = {inserted} (expect True)")
    assert inserted

    hs_len = hs.len()
    print(f"hs.len() after insert = {hs_len} (expect 1)")
    assert hs_len == 1

    # Call process_foreign_set which adds "from_process"
    hs = process_foreign_set(hs)
    hs_len = hs.len()
    print(f"hs.len() after process_foreign_set = {hs_len} (expect 2)")
    assert hs_len == 2

    # Verify actual values via contains() using make_foreign_string
    check_val = make_foreign_string(b"hello")
    found_hello = hs.contains(check_val)
    print(f"hs.contains('hello') = {found_hello} (expect True)")
    assert found_hello
    check_val.destroy()

    check_val = make_foreign_string(b"from_process")
    found_from_process = hs.contains(check_val)
    print(f"hs.contains('from_process') = {found_from_process} (expect True)")
    assert found_from_process
    check_val.destroy()

    check_val = make_foreign_string(b"world")
    found_world = hs.contains(check_val)
    print(f"hs.contains('world') = {found_world} (expect False)")
    assert not found_world
    check_val.destroy()

    hs.destroy()

    # ===== Final cleanup =====
    result.destroy()
    v.destroy()

    # ===== Async: make_name via wait =====
    print("\n--- Async: make_name via wait ---")
    rt = new_runtime()

    future = make_name(b"hello")
    async_result = future.wait(rt)
    async_len = async_result.len()
    print(f"make_name('hello').len() = {async_len} (expect 11)")
    assert async_len == 11

    async_str = async_result.as_str()
    async_first = async_str.get(0)
    print(f"make_name('hello')[0] = '{chr(async_first)}' (expect 'h')")
    assert chr(async_first) == 'h'
    async_str.destroy()
    async_result.destroy()

    # ===== Async: make_name via async_wait =====
    print("\n--- Async: make_name via async_wait ---")
    rt2 = new_runtime()

    ON_RETURN_STRING = ctypes.CFUNCTYPE(None, AbiClass_Foreign_String, ctypes.c_void_p)

    def on_return_string(val, ctx):
        ctypes.cast(ctx, ctypes.POINTER(AbiClass_Foreign_String))[0] = val

    future2 = make_name(b"async")
    result_slot = AbiClass_Foreign_String()
    notify2 = Notify_Foreign_String()
    notify2.on_return = ON_RETURN_STRING(on_return_string)
    notify2.ctx = ctypes.addressof(result_slot)
    future2.async_wait(rt2, notify2)

    async_len2 = result_slot.methods[0].len(ctypes.byref(result_slot))
    print(f"make_name('async') len = {async_len2} (expect 11)")
    assert async_len2 == 11

    # Wrap result_slot in Foreign_String RAII class for destroy
    result_str2 = Foreign_String(result_slot)
    result_str2.destroy()
    rt2.destroy()
    rt.destroy()

    print("\nForeign type example passed!")


def foreign_type_c():
    """Get the Hicc_foreign_type function table pointer."""
    from hicc_foreign_type import _lib
    _lib.foreign_type.restype = ctypes.POINTER(Hicc_foreign_type)
    return _lib.foreign_type()


if __name__ == "__main__":
    main()