cmake_minimum_required(VERSION 3.16)
project(hicc_usage_hello_world CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB hello_world_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(hello_world STATIC ${hello_world_SOURCES})
target_include_directories(hello_world PUBLIC include)

enable_testing()
add_executable(test_hello_world test/main.cpp)
target_link_libraries(test_hello_world PRIVATE hello_world)
add_test(NAME hello_world_test COMMAND test_hello_world)
