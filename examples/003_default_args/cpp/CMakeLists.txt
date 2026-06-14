cmake_minimum_required(VERSION 3.10)
project(default_args CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(default_args_cmake default_args.cpp main.cpp)
target_include_directories(default_args_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(default_args STATIC default_args.cpp)
target_include_directories(default_args PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
