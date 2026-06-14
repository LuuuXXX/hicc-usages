cmake_minimum_required(VERSION 3.10)
project(hello_world CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(hello_world_cmake
    hello_world.cpp
    main.cpp
)
target_include_directories(hello_world_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

# 静态库（供 Rust hicc-build 链接；standalone 不依赖此库）
add_library(hello_world STATIC hello_world.cpp)
target_include_directories(hello_world PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
