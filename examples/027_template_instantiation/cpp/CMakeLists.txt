cmake_minimum_required(VERSION 3.10)
project(template_instantiation CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(template_instantiation_cmake template_instantiation.cpp main.cpp)
target_include_directories(template_instantiation_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(template_instantiation STATIC template_instantiation.cpp)
target_include_directories(template_instantiation PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
