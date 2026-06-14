cmake_minimum_required(VERSION 3.16)
project(hicc_usage_tuple_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB tuple_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(tuple_basic STATIC ${tuple_basic_SOURCES})
target_include_directories(tuple_basic PUBLIC include)

enable_testing()
add_executable(test_tuple_basic test/main.cpp)
target_link_libraries(test_tuple_basic PRIVATE tuple_basic)
add_test(NAME tuple_basic_test COMMAND test_tuple_basic)
