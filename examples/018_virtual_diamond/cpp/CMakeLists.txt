cmake_minimum_required(VERSION 3.10)
project(virtual_diamond CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(virtual_diamond_cmake virtual_diamond.cpp main.cpp)
target_include_directories(virtual_diamond_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(virtual_diamond STATIC virtual_diamond.cpp)
target_include_directories(virtual_diamond PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
