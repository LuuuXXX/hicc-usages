cmake_minimum_required(VERSION 3.10)
project(class_const CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_const_cmake class_const.cpp main.cpp)
target_include_directories(class_const_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_const STATIC class_const.cpp)
target_include_directories(class_const PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
