from hicc_no_std_demo import *


def test_plain_i32():
    sum_val = add(3, 4)
    print(f"add(3, 4) = {sum_val}")

    neg = negate(5)
    print(f"negate(5) = {neg}")


def test_container_i32():
    c = new_container(42)
    val = c.get()
    print(f"container_value(42) = {val}")
    c.destroy()


def test_option_i32():
    opt = new_option(99)
    p = opt.as_ref()
    doubled = p * 2
    print(f"double_option(Some(99)) = {doubled}")
    opt.destroy()


def main():
    test_plain_i32()
    test_container_i32()
    test_option_i32()
    print("no_std demo passed!")


if __name__ == "__main__":
    main()