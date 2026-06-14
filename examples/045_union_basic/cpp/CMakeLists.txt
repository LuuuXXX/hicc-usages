cmake_minimum_required(VERSION 3.16)
project(hicc_usage_union_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB union_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(union_basic STATIC ${union_basic_SOURCES})
target_include_directories(union_basic PUBLIC include)

enable_testing()
add_executable(test_union_basic test/main.cpp)
target_link_libraries(test_union_basic PRIVATE union_basic)
add_test(NAME union_basic_test COMMAND test_union_basic)
