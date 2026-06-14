# CMakeLists.txt template — produces lib<name>.a in build/
# Replace noexcept_basic and noexcept_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(noexcept_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(noexcept_basic STATIC noexcept_basic.cpp)
target_include_directories(noexcept_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(noexcept_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(noexcept_basic_demo main.cpp)
    target_link_libraries(noexcept_basic_demo PRIVATE noexcept_basic)
endif()
