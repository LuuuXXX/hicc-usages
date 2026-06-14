cmake_minimum_required(VERSION 3.10)
project(noexcept_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(noexcept_basic_cmake noexcept_basic.cpp main.cpp)
target_include_directories(noexcept_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(noexcept_basic STATIC noexcept_basic.cpp)
target_include_directories(noexcept_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
