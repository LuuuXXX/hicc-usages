# CMakeLists.txt template — produces lib<name>.a in build/
# Replace operator_overload and operator_overload.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(operator_overload CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(operator_overload STATIC operator_overload.cpp)
target_include_directories(operator_overload PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(operator_overload PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(operator_overload_demo main.cpp)
    target_link_libraries(operator_overload_demo PRIVATE operator_overload)
endif()
