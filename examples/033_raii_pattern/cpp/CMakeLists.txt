cmake_minimum_required(VERSION 3.10)
project(raii_pattern CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(raii_pattern_cmake raii_pattern.cpp main.cpp)
target_include_directories(raii_pattern_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(raii_pattern STATIC raii_pattern.cpp)
target_include_directories(raii_pattern PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
