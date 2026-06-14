# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_const and class_const.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_const CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_const STATIC class_const.cpp)
target_include_directories(class_const PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_const PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_const_demo main.cpp)
    target_link_libraries(class_const_demo PRIVATE class_const)
endif()
