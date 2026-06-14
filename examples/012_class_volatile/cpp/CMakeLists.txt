cmake_minimum_required(VERSION 3.10)
project(class_volatile CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_volatile_cmake class_volatile.cpp main.cpp)
target_include_directories(class_volatile_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_volatile STATIC class_volatile.cpp)
target_include_directories(class_volatile PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
