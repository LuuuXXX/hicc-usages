# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_copy and class_copy.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_copy CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_copy STATIC class_copy.cpp)
target_include_directories(class_copy PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_copy PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_copy_demo main.cpp)
    target_link_libraries(class_copy_demo PRIVATE class_copy)
endif()
