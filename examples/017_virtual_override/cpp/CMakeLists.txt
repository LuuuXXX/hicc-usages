cmake_minimum_required(VERSION 3.10)
project(virtual_override CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(virtual_override_cmake virtual_override.cpp main.cpp)
target_include_directories(virtual_override_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(virtual_override STATIC virtual_override.cpp)
target_include_directories(virtual_override PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
