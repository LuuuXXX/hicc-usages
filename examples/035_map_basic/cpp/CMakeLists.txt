# CMakeLists.txt template — produces lib<name>.a in build/
# Replace map_basic and map_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(map_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(map_basic STATIC map_basic.cpp)
target_include_directories(map_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(map_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(map_basic_demo main.cpp)
    target_link_libraries(map_basic_demo PRIVATE map_basic)
endif()
