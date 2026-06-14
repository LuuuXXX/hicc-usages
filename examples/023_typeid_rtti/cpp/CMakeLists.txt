# CMakeLists.txt template — produces lib<name>.a in build/
# Replace typeid_rtti and typeid_rtti.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(typeid_rtti CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(typeid_rtti STATIC typeid_rtti.cpp)
target_include_directories(typeid_rtti PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(typeid_rtti PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(typeid_rtti_demo main.cpp)
    target_link_libraries(typeid_rtti_demo PRIVATE typeid_rtti)
endif()
