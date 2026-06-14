cmake_minimum_required(VERSION 3.10)
project(class_move CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_move_cmake class_move.cpp main.cpp)
target_include_directories(class_move_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_move STATIC class_move.cpp)
target_include_directories(class_move PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
