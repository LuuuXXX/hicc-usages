# CMakeLists.txt template — produces lib<name>.a in build/
# Replace functional_bind and functional_bind.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(functional_bind CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(functional_bind STATIC functional_bind.cpp)
target_include_directories(functional_bind PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(functional_bind PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(functional_bind_demo main.cpp)
    target_link_libraries(functional_bind_demo PRIVATE functional_bind)
endif()
