cmake_minimum_required(VERSION 3.10)
project(template_specialization CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(template_specialization_cmake template_specialization.cpp main.cpp)
target_include_directories(template_specialization_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(template_specialization STATIC template_specialization.cpp)
target_include_directories(template_specialization PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
