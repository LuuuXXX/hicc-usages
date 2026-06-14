# CMakeLists.txt template — produces lib<name>.a in build/
# Replace raii_pattern and raii_pattern.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(raii_pattern CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(raii_pattern STATIC raii_pattern.cpp)
target_include_directories(raii_pattern PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(raii_pattern PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(raii_pattern_demo main.cpp)
    target_link_libraries(raii_pattern_demo PRIVATE raii_pattern)
endif()
