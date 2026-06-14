cmake_minimum_required(VERSION 3.10)
project(class_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_basic_cmake class_basic.cpp main.cpp)
target_include_directories(class_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_basic STATIC class_basic.cpp)
target_include_directories(class_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
