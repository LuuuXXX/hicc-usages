cmake_minimum_required(VERSION 3.10)
project(shared_ptr CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(shared_ptr_cmake shared_ptr.cpp main.cpp)
target_include_directories(shared_ptr_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(shared_ptr STATIC shared_ptr.cpp)
target_include_directories(shared_ptr PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
