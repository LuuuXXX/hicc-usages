cmake_minimum_required(VERSION 3.10)
project(inheritance_single CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(inheritance_single_cmake inheritance_single.cpp main.cpp)
target_include_directories(inheritance_single_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(inheritance_single STATIC inheritance_single.cpp)
target_include_directories(inheritance_single PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
