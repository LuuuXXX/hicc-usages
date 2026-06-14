# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_basic and class_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_basic STATIC class_basic.cpp)
target_include_directories(class_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_basic_demo main.cpp)
    target_link_libraries(class_basic_demo PRIVATE class_basic)
endif()
