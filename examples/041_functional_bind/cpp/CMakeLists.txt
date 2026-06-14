cmake_minimum_required(VERSION 3.10)
project(functional_bind CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(functional_bind_cmake functional_bind.cpp main.cpp)
target_include_directories(functional_bind_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(functional_bind STATIC functional_bind.cpp)
target_include_directories(functional_bind PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
