# CMakeLists.txt template — produces lib<name>.a in build/
# Replace exception_basic and exception_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(exception_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(exception_basic STATIC exception_basic.cpp)
target_include_directories(exception_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(exception_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(exception_basic_demo main.cpp)
    target_link_libraries(exception_basic_demo PRIVATE exception_basic)
endif()
