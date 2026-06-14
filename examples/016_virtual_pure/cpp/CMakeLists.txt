cmake_minimum_required(VERSION 3.10)
project(virtual_pure CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(virtual_pure_cmake virtual_pure.cpp main.cpp)
target_include_directories(virtual_pure_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(virtual_pure STATIC virtual_pure.cpp)
target_include_directories(virtual_pure PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
