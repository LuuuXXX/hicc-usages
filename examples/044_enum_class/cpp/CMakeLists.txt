# CMakeLists.txt template — produces lib<name>.a in build/
# Replace enum_class and enum_class.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(enum_class CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(enum_class STATIC enum_class.cpp)
target_include_directories(enum_class PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(enum_class PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(enum_class_demo main.cpp)
    target_link_libraries(enum_class_demo PRIVATE enum_class)
endif()
