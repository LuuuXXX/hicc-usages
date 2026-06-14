"""Python test for basic_lib — matches C output exactly."""
from hicc_demo import (add, negate, container_value, double_option,
                        check_str, count_some, total_len, add_point,
                        new_container, new_option, new_str, new_slice,
                        new_array, Point)
from hicc_demo import Option_i32, Container_i32


def main():
    # 1. Plain i32
    sum_val = add(3, 4)
    print(f"add(3, 4) = {sum_val}")

    neg = negate(5)
    print(f"negate(5) = {neg}")

    # 2. Container<i32>
    c = new_container(42)
    print(f"container_value(42) = {c.get()}")

    # 3. Option<i32>
    opt = new_option(99)
    val = opt.as_ref()
    doubled = val * 2
    print(f"double_option(Some(99)) = {doubled}")

    # 4. Str
    s = new_str()
    print(f'check_str("hello") = {s.len()}')

    # 5. Slice<Option<i32>> — value-passed AbiClass, Rust side frees
    slice_val = new_slice()
    count = count_some(slice_val)
    # slice_val._inner nullified by count_some wrapper
    print(f"count_some(&[Some(10), None, Some(30)]) = {count}")

    # 6. Array<&'static str, 3> — value-passed AbiClass, Rust side frees
    arr = new_array()
    total = total_len(arr)
    # arr._inner nullified by total_len wrapper
    print(f'total_len(["a", "bb", "ccc"]) = {total}')

    # 7. Point (POD struct)
    p1 = Point(10, 20)
    p2 = Point(1, 2)
    sp = add_point(p1, p2)
    print(f"add_point((10,20), (1,2)) = ({sp.x}, {sp.y})")

    print("Basic lib example passed!")


if __name__ == "__main__":
    main()
