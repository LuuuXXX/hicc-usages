cmake_minimum_required(VERSION 3.10)
project(class_copy CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_copy_cmake class_copy.cpp main.cpp)
target_include_directories(class_copy_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_copy STATIC class_copy.cpp)
target_include_directories(class_copy PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
