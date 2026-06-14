# CMakeLists.txt template — produces lib<name>.a in build/
# Replace virtual_basic and virtual_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(virtual_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(virtual_basic STATIC virtual_basic.cpp)
target_include_directories(virtual_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(virtual_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(virtual_basic_demo main.cpp)
    target_link_libraries(virtual_basic_demo PRIVATE virtual_basic)
endif()
