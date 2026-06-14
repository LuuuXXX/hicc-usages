cmake_minimum_required(VERSION 3.10)
project(variadic_template CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(variadic_template_cmake variadic_template.cpp main.cpp)
target_include_directories(variadic_template_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(variadic_template STATIC variadic_template.cpp)
target_include_directories(variadic_template PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
