cmake_minimum_required(VERSION 3.10)
project(constexpr_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(constexpr_basic_cmake constexpr_basic.cpp main.cpp)
target_include_directories(constexpr_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(constexpr_basic STATIC constexpr_basic.cpp)
target_include_directories(constexpr_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
