# CMakeLists.txt template — produces lib<name>.a in build/
# Replace variadic_template and variadic_template.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(variadic_template CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(variadic_template STATIC variadic_template.cpp)
target_include_directories(variadic_template PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(variadic_template PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(variadic_template_demo main.cpp)
    target_link_libraries(variadic_template_demo PRIVATE variadic_template)
endif()
