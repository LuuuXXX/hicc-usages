import ctypes
import sys
import os

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
os.environ["LD_LIBRARY_PATH"] = os.path.dirname(os.path.abspath(__file__)) + "/../" + os.environ.get("LD_LIBRARY_PATH", "")

from async_func import *

ON_RETURN_I32 = ctypes.CFUNCTYPE(None, ctypes.c_int32, ctypes.c_void_p)
ON_RETURN_STRING = ctypes.CFUNCTYPE(None, AbiClass_String, ctypes.c_void_p)
ON_RETURN_STRING_NON_FOREIGN = ctypes.CFUNCTYPE(None, AbiClass_String, ctypes.c_void_p)

def main():
    print("=== Async-func Python example ===")

    rt = new_runtime()
    print(f"Runtime created")

    # Test 1: async_add via wait (blocking)
    print("\n--- async_add(3, 4) via wait ---")
    future1 = async_add(3, 4)
    result1 = future1.wait(rt)
    print(f"async_add(3, 4) = {result1} (expect 7)")
    assert result1 == 7

    # Test 2: async_add via async_wait (callback)
    print("\n--- async_add(10, 20) via async_wait ---")
    future2 = async_add(10, 20)
    result_slot = ctypes.c_int32(0)

    def on_return_i32(val, ctx):
        ctypes.cast(ctx, ctypes.POINTER(ctypes.c_int32))[0] = val

    notify_i32 = Notify_Foreign_i32()
    notify_i32.on_return = ON_RETURN_I32(on_return_i32)
    notify_i32.ctx = ctypes.addressof(result_slot)
    future2.async_wait(rt, notify_i32)
    print(f"async_add(10, 20) = {result_slot.value} (expect 30)")
    assert result_slot.value == 30

    # Test 3: async_hello via wait (blocking)
    print("\n--- async_hello via wait ---")
    future3 = async_hello()
    result_str = future3.wait(rt)
    str_len = result_str.len()
    print(f"async_hello().len() = {str_len} (expect 11)")
    assert str_len == 11
    str_ref = result_str.as_str()
    first_char = str_ref.get(0)
    print(f"async_hello()[0] = '{chr(first_char)}' (expect 'h')")
    assert chr(first_char) == 'h'
    str_ref.destroy()
    result_str.destroy()

    # Test 4: async_hello via async_wait (callback)
    print("\n--- async_hello via async_wait ---")
    future4 = async_hello()
    result_str_slot = AbiClass_String()

    def on_return_string(val, ctx):
        ctypes.cast(ctx, ctypes.POINTER(AbiClass_String))[0] = val

    notify_string = Notify_Foreign_String()
    notify_string.on_return = ON_RETURN_STRING(on_return_string)
    notify_string.ctx = ctypes.addressof(result_str_slot)
    future4.async_wait(rt, notify_string)
    result_str2 = String(result_str_slot)
    str_len2 = result_str2.len()
    print(f"async_hello (async_wait).len() = {str_len2} (expect 11)")
    assert str_len2 == 11
    result_str2.destroy()

    # Test 5: AsyncCounter.async_increment via wait (blocking)
    print("\n--- AsyncCounter.async_increment(5) via wait ---")
    counter = new_counter(100)
    future_inc = counter.async_increment(5)
    result_inc = future_inc.wait(rt)
    print(f"async_increment(5) = {result_inc} (expect 105)")
    assert result_inc == 105
    future_inc.destroy()
    counter.destroy()

    # Test 6: AsyncCounter.async_greet via async_wait (callback)
    print("\n--- AsyncCounter.async_greet via async_wait ---")
    counter2 = new_counter(42)
    future_greet = counter2.async_greet()
    result_greet_slot = AbiClass_String()

    def on_return_string_non_foreign(val, ctx):
        ctypes.cast(ctx, ctypes.POINTER(AbiClass_String))[0] = val

    notify_greet = Notify_String()
    notify_greet.on_return = ON_RETURN_STRING_NON_FOREIGN(on_return_string_non_foreign)
    notify_greet.ctx = ctypes.addressof(result_greet_slot)
    future_greet.async_wait(rt, notify_greet)
    greet_str = String(result_greet_slot)
    greet_len = greet_str.len()
    print(f"async_greet().len() = {greet_len} (expect 13)")
    assert greet_len == 13
    greet_str.destroy()
    future_greet.destroy()
    counter2.destroy()

    rt.destroy()
    print("\nRuntime destroyed")
    print("\nAsync-func Python example passed!")


if __name__ == "__main__":
    main()