# CMakeLists.txt template — produces lib<name>.a in build/
# Replace explicit_ctor and explicit_ctor.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(explicit_ctor CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(explicit_ctor STATIC explicit_ctor.cpp)
target_include_directories(explicit_ctor PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(explicit_ctor PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(explicit_ctor_demo main.cpp)
    target_link_libraries(explicit_ctor_demo PRIVATE explicit_ctor)
endif()
