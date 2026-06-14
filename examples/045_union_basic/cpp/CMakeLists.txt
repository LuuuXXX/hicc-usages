# CMakeLists.txt template — produces lib<name>.a in build/
# Replace union_basic and union_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(union_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(union_basic STATIC union_basic.cpp)
target_include_directories(union_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(union_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(union_basic_demo main.cpp)
    target_link_libraries(union_basic_demo PRIVATE union_basic)
endif()
