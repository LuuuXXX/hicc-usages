cmake_minimum_required(VERSION 3.10)
project(vector_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(vector_basic_cmake vector_basic.cpp main.cpp)
target_include_directories(vector_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(vector_basic STATIC vector_basic.cpp)
target_include_directories(vector_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
