# CMakeLists.txt template — produces lib<name>.a in build/
# Replace vector_basic and vector_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(vector_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(vector_basic STATIC vector_basic.cpp)
target_include_directories(vector_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(vector_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(vector_basic_demo main.cpp)
    target_link_libraries(vector_basic_demo PRIVATE vector_basic)
endif()
