cmake_minimum_required(VERSION 3.10)
project(tuple_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(tuple_basic_cmake tuple_basic.cpp main.cpp)
target_include_directories(tuple_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(tuple_basic STATIC tuple_basic.cpp)
target_include_directories(tuple_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
