# CMakeLists.txt template — produces lib<name>.a in build/
# Replace placement_new and placement_new.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(placement_new CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(placement_new STATIC placement_new.cpp)
target_include_directories(placement_new PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(placement_new PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(placement_new_demo main.cpp)
    target_link_libraries(placement_new_demo PRIVATE placement_new)
endif()
