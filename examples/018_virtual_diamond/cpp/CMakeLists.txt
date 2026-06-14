# CMakeLists.txt template — produces lib<name>.a in build/
# Replace virtual_diamond and virtual_diamond.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(virtual_diamond CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(virtual_diamond STATIC virtual_diamond.cpp)
target_include_directories(virtual_diamond PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(virtual_diamond PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(virtual_diamond_demo main.cpp)
    target_link_libraries(virtual_diamond_demo PRIVATE virtual_diamond)
endif()
