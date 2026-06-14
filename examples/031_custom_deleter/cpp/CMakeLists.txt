# CMakeLists.txt template — produces lib<name>.a in build/
# Replace custom_deleter and custom_deleter.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(custom_deleter CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(custom_deleter STATIC custom_deleter.cpp)
target_include_directories(custom_deleter PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(custom_deleter PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(custom_deleter_demo main.cpp)
    target_link_libraries(custom_deleter_demo PRIVATE custom_deleter)
endif()
