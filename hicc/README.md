# hicc

##

包含以下项目:
1. [`hicc`](hicc/README.md): `c++`接口转换为`rust api`的基础功能
1. [`hicc-build`](hicc-build/src/lib.rs): 基于`hicc`的二次开发包含`c++`代码，提供构建`c++`代码的功能.
1. [`hicc-std`](hicc-std/README.md): 基于`hicc`将`c++`标准库中容器操作的大部分接口转换为`rust api`
1. [`hicc-rs`](hicc-rs/README.md): `Rust`接口转换为`capi`
1. [`hicc-cbindgen`](hicc-cbindgen/README.md): 生成`Rust`接口转换为`capi`接口对应的C头文件.

使用方式参考[使用指南](https://gitcode.com/xuanwu/hicc/blob/main/reference.md).

