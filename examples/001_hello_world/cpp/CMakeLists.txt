# CMakeLists.txt template — produces lib<name>.a in build/
# Replace hello_world and hello_world.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(hello_world CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(hello_world STATIC hello_world.cpp)
target_include_directories(hello_world PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(hello_world PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(hello_world_demo main.cpp)
    target_link_libraries(hello_world_demo PRIVATE hello_world)
endif()
