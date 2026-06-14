# CMakeLists.txt template — produces lib<name>.a in build/
# Replace mutable_member and mutable_member.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(mutable_member CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(mutable_member STATIC mutable_member.cpp)
target_include_directories(mutable_member PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(mutable_member PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(mutable_member_demo main.cpp)
    target_link_libraries(mutable_member_demo PRIVATE mutable_member)
endif()
