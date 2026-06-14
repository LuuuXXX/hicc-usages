cmake_minimum_required(VERSION 3.10)
project(placement_new CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(placement_new_cmake placement_new.cpp main.cpp)
target_include_directories(placement_new_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(placement_new STATIC placement_new.cpp)
target_include_directories(placement_new PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
