cmake_minimum_required(VERSION 3.10)
project(exception_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(exception_basic_cmake exception_basic.cpp main.cpp)
target_include_directories(exception_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(exception_basic STATIC exception_basic.cpp)
target_include_directories(exception_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
