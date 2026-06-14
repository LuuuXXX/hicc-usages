# CMakeLists.txt template — produces lib<name>.a in build/
# Replace std_function and std_function.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(std_function CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(std_function STATIC std_function.cpp)
target_include_directories(std_function PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(std_function PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(std_function_demo main.cpp)
    target_link_libraries(std_function_demo PRIVATE std_function)
endif()
