# CMakeLists.txt template — produces lib<name>.a in build/
# Replace array_basic and array_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(array_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(array_basic STATIC array_basic.cpp)
target_include_directories(array_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(array_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(array_basic_demo main.cpp)
    target_link_libraries(array_basic_demo PRIVATE array_basic)
endif()
