# CMakeLists.txt template — produces lib<name>.a in build/
# Replace template_instantiation and template_instantiation.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(template_instantiation CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(template_instantiation STATIC template_instantiation.cpp)
target_include_directories(template_instantiation PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(template_instantiation PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(template_instantiation_demo main.cpp)
    target_link_libraries(template_instantiation_demo PRIVATE template_instantiation)
endif()
