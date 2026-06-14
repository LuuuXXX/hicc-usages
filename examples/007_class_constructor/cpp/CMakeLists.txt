# CMakeLists.txt template — produces lib<name>.a in build/
# Replace class_constructor and class_constructor.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(class_constructor CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(class_constructor STATIC class_constructor.cpp)
target_include_directories(class_constructor PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(class_constructor PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(class_constructor_demo main.cpp)
    target_link_libraries(class_constructor_demo PRIVATE class_constructor)
endif()
