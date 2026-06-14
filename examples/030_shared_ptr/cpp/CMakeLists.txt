# CMakeLists.txt template — produces lib<name>.a in build/
# Replace shared_ptr and shared_ptr.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(shared_ptr CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(shared_ptr STATIC shared_ptr.cpp)
target_include_directories(shared_ptr PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(shared_ptr PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(shared_ptr_demo main.cpp)
    target_link_libraries(shared_ptr_demo PRIVATE shared_ptr)
endif()
