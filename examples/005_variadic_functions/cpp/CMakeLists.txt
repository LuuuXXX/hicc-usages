cmake_minimum_required(VERSION 3.10)
project(variadic_functions CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(variadic_functions_cmake variadic_functions.cpp main.cpp)
target_include_directories(variadic_functions_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(variadic_functions STATIC variadic_functions.cpp)
target_include_directories(variadic_functions PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
