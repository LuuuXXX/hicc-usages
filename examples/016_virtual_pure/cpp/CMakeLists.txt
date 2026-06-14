# CMakeLists.txt template — produces lib<name>.a in build/
# Replace virtual_pure and virtual_pure.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(virtual_pure CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(virtual_pure STATIC virtual_pure.cpp)
target_include_directories(virtual_pure PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(virtual_pure PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(virtual_pure_demo main.cpp)
    target_link_libraries(virtual_pure_demo PRIVATE virtual_pure)
endif()
