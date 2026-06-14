# CMakeLists.txt template — produces lib<name>.a in build/
# Replace template_specialization and template_specialization.cpp placeholders when scaffolding.
cmake_minimum_required(VERSION 3.10)
project(template_specialization CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(template_specialization STATIC template_specialization.cpp)
target_include_directories(template_specialization PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(template_specialization PRIVATE -Wall -Wextra -fPIC)

# Demo executable (verify the library actually works in pure C++).
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    add_executable(template_specialization_demo main.cpp)
    target_link_libraries(template_specialization_demo PRIVATE template_specialization)
endif()
