cmake_minimum_required(VERSION 3.16)
project(hicc_usage_array_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB array_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(array_basic STATIC ${array_basic_SOURCES})
target_include_directories(array_basic PUBLIC include)

enable_testing()
add_executable(test_array_basic test/main.cpp)
target_link_libraries(test_array_basic PRIVATE array_basic)
add_test(NAME array_basic_test COMMAND test_array_basic)
