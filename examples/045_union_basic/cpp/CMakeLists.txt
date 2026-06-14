cmake_minimum_required(VERSION 3.10)
project(union_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(union_basic_cmake union_basic.cpp main.cpp)
target_include_directories(union_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(union_basic STATIC union_basic.cpp)
target_include_directories(union_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
