cmake_minimum_required(VERSION 3.10)
project(template_function CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(template_function_cmake template_function.cpp main.cpp)
target_include_directories(template_function_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(template_function STATIC template_function.cpp)
target_include_directories(template_function PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
