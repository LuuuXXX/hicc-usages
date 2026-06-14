cmake_minimum_required(VERSION 3.16)
project(hicc_usage_class_const CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB class_const_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(class_const STATIC ${class_const_SOURCES})
target_include_directories(class_const PUBLIC include)

enable_testing()
add_executable(test_class_const test/main.cpp)
target_link_libraries(test_class_const PRIVATE class_const)
add_test(NAME class_const_test COMMAND test_class_const)
