cmake_minimum_required(VERSION 3.10)
project(mutable_member CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(mutable_member_cmake mutable_member.cpp main.cpp)
target_include_directories(mutable_member_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(mutable_member STATIC mutable_member.cpp)
target_include_directories(mutable_member PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
