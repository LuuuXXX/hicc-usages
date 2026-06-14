# CMakeLists.txt template — produces lib<name>.a in build/
# Replace inline_functions and inline_functions.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(inline_functions CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(inline_functions STATIC inline_functions.cpp)
target_include_directories(inline_functions PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(inline_functions PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(inline_functions_demo main.cpp)
    target_link_libraries(inline_functions_demo PRIVATE inline_functions)
endif()
