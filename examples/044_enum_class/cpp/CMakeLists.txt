cmake_minimum_required(VERSION 3.10)
project(enum_class CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(enum_class_cmake enum_class.cpp main.cpp)
target_include_directories(enum_class_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(enum_class STATIC enum_class.cpp)
target_include_directories(enum_class PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
