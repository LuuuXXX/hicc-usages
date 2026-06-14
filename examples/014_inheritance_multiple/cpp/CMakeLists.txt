# CMakeLists.txt template — produces lib<name>.a in build/
# Replace inheritance_multiple and inheritance_multiple.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(inheritance_multiple CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(inheritance_multiple STATIC inheritance_multiple.cpp)
target_include_directories(inheritance_multiple PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(inheritance_multiple PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(inheritance_multiple_demo main.cpp)
    target_link_libraries(inheritance_multiple_demo PRIVATE inheritance_multiple)
endif()
