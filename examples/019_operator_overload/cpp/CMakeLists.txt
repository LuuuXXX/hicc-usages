cmake_minimum_required(VERSION 3.10)
project(operator_overload CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(operator_overload_cmake operator_overload.cpp main.cpp)
target_include_directories(operator_overload_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(operator_overload STATIC operator_overload.cpp)
target_include_directories(operator_overload PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
