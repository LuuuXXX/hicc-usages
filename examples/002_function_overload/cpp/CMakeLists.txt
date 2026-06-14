cmake_minimum_required(VERSION 3.10)
project(function_overload CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(function_overload_cmake function_overload.cpp main.cpp)
target_include_directories(function_overload_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(function_overload STATIC function_overload.cpp)
target_include_directories(function_overload PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
