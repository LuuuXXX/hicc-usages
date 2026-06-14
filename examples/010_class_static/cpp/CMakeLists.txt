# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_static and class_static.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_static CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_static STATIC class_static.cpp)
target_include_directories(class_static PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_static PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_static_demo main.cpp)
    target_link_libraries(class_static_demo PRIVATE class_static)
endif()
