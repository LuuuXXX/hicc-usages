cmake_minimum_required(VERSION 3.16)
project(hicc_usage_namespace_nested CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB namespace_nested_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(namespace_nested STATIC ${namespace_nested_SOURCES})
target_include_directories(namespace_nested PUBLIC include)

enable_testing()
add_executable(test_namespace_nested test/main.cpp)
target_link_libraries(test_namespace_nested PRIVATE namespace_nested)
add_test(NAME namespace_nested_test COMMAND test_namespace_nested)
