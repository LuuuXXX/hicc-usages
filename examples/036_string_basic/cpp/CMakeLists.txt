cmake_minimum_required(VERSION 3.10)
project(string_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(string_basic_cmake string_basic.cpp main.cpp)
target_include_directories(string_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(string_basic STATIC string_basic.cpp)
target_include_directories(string_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
