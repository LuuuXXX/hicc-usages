cmake_minimum_required(VERSION 3.10)
project(summary CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(summary_cmake summary.cpp main.cpp)
target_include_directories(summary_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(summary STATIC summary.cpp)
target_include_directories(summary PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
