# CMakeLists.txt template — produces lib<name>.a in build/
# Replace string_basic and string_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(string_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(string_basic STATIC string_basic.cpp)
target_include_directories(string_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(string_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(string_basic_demo main.cpp)
    target_link_libraries(string_basic_demo PRIVATE string_basic)
endif()
