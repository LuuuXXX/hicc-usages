cmake_minimum_required(VERSION 3.10)
project(typeid_rtti CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(typeid_rtti_cmake typeid_rtti.cpp main.cpp)
target_include_directories(typeid_rtti_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(typeid_rtti STATIC typeid_rtti.cpp)
target_include_directories(typeid_rtti PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
