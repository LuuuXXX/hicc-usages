# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_volatile and class_volatile.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_volatile CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_volatile STATIC class_volatile.cpp)
target_include_directories(class_volatile PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_volatile PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_volatile_demo main.cpp)
    target_link_libraries(class_volatile_demo PRIVATE class_volatile)
endif()
