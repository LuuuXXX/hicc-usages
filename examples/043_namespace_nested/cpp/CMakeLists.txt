cmake_minimum_required(VERSION 3.10)
project(namespace_nested CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(namespace_nested_cmake namespace_nested.cpp main.cpp)
target_include_directories(namespace_nested_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(namespace_nested STATIC namespace_nested.cpp)
target_include_directories(namespace_nested PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
