# CMakeLists.txt template — produces lib<name>.a in build/
# Replace inheritance_single and inheritance_single.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(inheritance_single CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(inheritance_single STATIC inheritance_single.cpp)
target_include_directories(inheritance_single PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(inheritance_single PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(inheritance_single_demo main.cpp)
    target_link_libraries(inheritance_single_demo PRIVATE inheritance_single)
endif()
