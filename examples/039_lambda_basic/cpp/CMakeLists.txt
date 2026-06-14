# CMakeLists.txt template — produces lib<name>.a in build/
# Replace lambda_basic and lambda_basic.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(lambda_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(lambda_basic STATIC lambda_basic.cpp)
target_include_directories(lambda_basic PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(lambda_basic PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(lambda_basic_demo main.cpp)
    target_link_libraries(lambda_basic_demo PRIVATE lambda_basic)
endif()
