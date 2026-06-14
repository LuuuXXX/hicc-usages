cmake_minimum_required(VERSION 3.10)
project(unique_ptr CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(unique_ptr_cmake unique_ptr.cpp main.cpp)
target_include_directories(unique_ptr_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(unique_ptr STATIC unique_ptr.cpp)
target_include_directories(unique_ptr PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
