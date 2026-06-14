# CMakeLists.txt template — produces lib<name>.a in build/
# Replace friend_function and friend_function.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(friend_function CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(friend_function STATIC friend_function.cpp)
target_include_directories(friend_function PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(friend_function PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(friend_function_demo main.cpp)
    target_link_libraries(friend_function_demo PRIVATE friend_function)
endif()
