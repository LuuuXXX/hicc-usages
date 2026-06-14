cmake_minimum_required(VERSION 3.10)
project(template_class CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(template_class_cmake template_class.cpp main.cpp)
target_include_directories(template_class_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(template_class STATIC template_class.cpp)
target_include_directories(template_class PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
