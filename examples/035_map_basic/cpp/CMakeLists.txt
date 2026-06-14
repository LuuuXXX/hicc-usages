cmake_minimum_required(VERSION 3.10)
project(map_basic CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(map_basic_cmake map_basic.cpp main.cpp)
target_include_directories(map_basic_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(map_basic STATIC map_basic.cpp)
target_include_directories(map_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
