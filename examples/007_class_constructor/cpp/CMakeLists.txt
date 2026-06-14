cmake_minimum_required(VERSION 3.10)
project(class_constructor CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(class_constructor_cmake class_constructor.cpp main.cpp)
target_include_directories(class_constructor_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(class_constructor STATIC class_constructor.cpp)
target_include_directories(class_constructor PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
