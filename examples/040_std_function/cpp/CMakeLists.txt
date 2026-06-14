cmake_minimum_required(VERSION 3.10)
project(std_function CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(std_function_cmake std_function.cpp main.cpp)
target_include_directories(std_function_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(std_function STATIC std_function.cpp)
target_include_directories(std_function PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
