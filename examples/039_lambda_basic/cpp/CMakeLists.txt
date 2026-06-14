cmake_minimum_required(VERSION 3.10)
project(lambda_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(lambda_basic_cmake lambda_basic.cpp main.cpp)
target_include_directories(lambda_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(lambda_basic STATIC lambda_basic.cpp)
target_include_directories(lambda_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
