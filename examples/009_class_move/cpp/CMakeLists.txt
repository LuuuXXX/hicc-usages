# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_move and class_move.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_move CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_move STATIC class_move.cpp)
target_include_directories(class_move PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_move PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_move_demo main.cpp)
    target_link_libraries(class_move_demo PRIVATE class_move)
endif()
