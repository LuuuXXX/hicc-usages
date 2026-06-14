# CMakeLists.txt template — produces lib<name>.a in build/
# Replace function_overload and function_overload.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(function_overload CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(function_overload STATIC function_overload.cpp)
target_include_directories(function_overload PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(function_overload PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(function_overload_demo main.cpp)
    target_link_libraries(function_overload_demo PRIVATE function_overload)
endif()
