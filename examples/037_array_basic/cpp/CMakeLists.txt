cmake_minimum_required(VERSION 3.10)
project(array_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(array_basic_cmake array_basic.cpp main.cpp)
target_include_directories(array_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(array_basic STATIC array_basic.cpp)
target_include_directories(array_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
