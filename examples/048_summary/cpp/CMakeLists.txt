# CMakeLists.txt template — produces lib<name>.a in build/
# Replace summary and summary.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(summary CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(summary STATIC summary.cpp)
target_include_directories(summary PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(summary PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(summary_demo main.cpp)
    target_link_libraries(summary_demo PRIVATE summary)
endif()
