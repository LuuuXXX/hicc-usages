cmake_minimum_required(VERSION 3.10)
project(explicit_ctor CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(explicit_ctor_cmake explicit_ctor.cpp main.cpp)
target_include_directories(explicit_ctor_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(explicit_ctor STATIC explicit_ctor.cpp)
target_include_directories(explicit_ctor PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
