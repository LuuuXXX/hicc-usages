# CMakeLists.txt template — produces lib<name>.a in build/
# Replace template_class and template_class.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(template_class CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(template_class STATIC template_class.cpp)
target_include_directories(template_class PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(template_class PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(template_class_demo main.cpp)
    target_link_libraries(template_class_demo PRIVATE template_class)
endif()
