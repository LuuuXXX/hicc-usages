cmake_minimum_required(VERSION 3.10)
project(inheritance_multiple CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(inheritance_multiple_cmake inheritance_multiple.cpp main.cpp)
target_include_directories(inheritance_multiple_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(inheritance_multiple STATIC inheritance_multiple.cpp)
target_include_directories(inheritance_multiple PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
