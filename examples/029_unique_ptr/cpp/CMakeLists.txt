# CMakeLists.txt template — produces lib<name>.a in build/
# Replace unique_ptr and unique_ptr.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(unique_ptr CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(unique_ptr STATIC unique_ptr.cpp)
target_include_directories(unique_ptr PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(unique_ptr PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(unique_ptr_demo main.cpp)
    target_link_libraries(unique_ptr_demo PRIVATE unique_ptr)
endif()
