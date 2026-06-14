cmake_minimum_required(VERSION 3.10)
project(friend_function CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(friend_function_cmake friend_function.cpp main.cpp)
target_include_directories(friend_function_cmake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(friend_function STATIC friend_function.cpp)
target_include_directories(friend_function PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
