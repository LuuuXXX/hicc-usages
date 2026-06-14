# CMakeLists.txt template — produces lib<name>.a in build/
# Replace namespace_nested and namespace_nested.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(namespace_nested CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(namespace_nested STATIC namespace_nested.cpp)
target_include_directories(namespace_nested PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(namespace_nested PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(namespace_nested_demo main.cpp)
    target_link_libraries(namespace_nested_demo PRIVATE namespace_nested)
endif()
