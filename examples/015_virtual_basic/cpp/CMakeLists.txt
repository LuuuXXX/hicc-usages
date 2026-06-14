cmake_minimum_required(VERSION 3.16)
project(hicc_usage_virtual_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB virtual_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(virtual_basic STATIC ${virtual_basic_SOURCES})
target_include_directories(virtual_basic PUBLIC include)

enable_testing()
add_executable(test_virtual_basic test/main.cpp)
target_link_libraries(test_virtual_basic PRIVATE virtual_basic)
add_test(NAME virtual_basic_test COMMAND test_virtual_basic)
