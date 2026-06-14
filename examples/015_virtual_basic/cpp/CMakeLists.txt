cmake_minimum_required(VERSION 3.10)
project(virtual_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(virtual_basic_cmake virtual_basic.cpp main.cpp)
target_include_directories(virtual_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(virtual_basic STATIC virtual_basic.cpp)
target_include_directories(virtual_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
