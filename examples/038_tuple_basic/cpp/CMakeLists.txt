# CMakeLists.txt template — produces lib<name>.a in build/
# Replace tuple_basic and tuple_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(tuple_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(tuple_basic STATIC tuple_basic.cpp)
target_include_directories(tuple_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(tuple_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(tuple_basic_demo main.cpp)
    target_link_libraries(tuple_basic_demo PRIVATE tuple_basic)
endif()
