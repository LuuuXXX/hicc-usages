cmake_minimum_required(VERSION 3.10)
project(custom_deleter CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(custom_deleter_cmake custom_deleter.cpp main.cpp)
target_include_directories(custom_deleter_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(custom_deleter STATIC custom_deleter.cpp)
target_include_directories(custom_deleter PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
