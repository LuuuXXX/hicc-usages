cmake_minimum_required(VERSION 3.10)
project(inline_functions CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(inline_functions_cmake inline_functions.cpp main.cpp)
target_include_directories(inline_functions_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(inline_functions STATIC inline_functions.cpp)
target_include_directories(inline_functions PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
