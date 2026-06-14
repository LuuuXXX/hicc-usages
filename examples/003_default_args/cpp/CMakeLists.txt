# CMakeLists.txt template — produces lib<name>.a in build/
# Replace default_args and default_args.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(default_args CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(default_args STATIC default_args.cpp)
target_include_directories(default_args PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(default_args PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(default_args_demo main.cpp)
    target_link_libraries(default_args_demo PRIVATE default_args)
endif()
