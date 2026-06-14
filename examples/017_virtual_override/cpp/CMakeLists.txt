# CMakeLists.txt template — produces lib<name>.a in build/
# Replace virtual_override and virtual_override.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(virtual_override CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(virtual_override STATIC virtual_override.cpp)
target_include_directories(virtual_override PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(virtual_override PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(virtual_override_demo main.cpp)
    target_link_libraries(virtual_override_demo PRIVATE virtual_override)
endif()
