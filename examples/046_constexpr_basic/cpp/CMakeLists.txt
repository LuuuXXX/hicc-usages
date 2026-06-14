# CMakeLists.txt template — produces lib<name>.a in build/
# Replace constexpr_basic and constexpr_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(constexpr_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(constexpr_basic STATIC constexpr_basic.cpp)
target_include_directories(constexpr_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(constexpr_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(constexpr_basic_demo main.cpp)
    target_link_libraries(constexpr_basic_demo PRIVATE constexpr_basic)
endif()
