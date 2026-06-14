# CMakeLists.txt template — produces lib<name>.a in build/
# Replace template_function and template_function.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(template_function CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(template_function STATIC template_function.cpp)
target_include_directories(template_function PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(template_function PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(template_function_demo main.cpp)
    target_link_libraries(template_function_demo PRIVATE template_function)
endif()
