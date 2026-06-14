cmake_minimum_required(VERSION 3.10)
project(class_static CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_static_cmake class_static.cpp main.cpp)
target_include_directories(class_static_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_static STATIC class_static.cpp)
target_include_directories(class_static PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
