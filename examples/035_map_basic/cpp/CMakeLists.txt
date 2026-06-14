cmake_minimum_required(VERSION 3.16)
project(hicc_usage_map_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB map_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(map_basic STATIC ${map_basic_SOURCES})
target_include_directories(map_basic PUBLIC include)

enable_testing()
add_executable(test_map_basic test/main.cpp)
target_link_libraries(test_map_basic PRIVATE map_basic)
add_test(NAME map_basic_test COMMAND test_map_basic)
