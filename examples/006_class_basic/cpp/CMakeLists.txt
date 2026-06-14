cmake_minimum_required(VERSION 3.16)
project(hicc_usage_class_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB class_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(class_basic STATIC ${class_basic_SOURCES})
target_include_directories(class_basic PUBLIC include)

enable_testing()
add_executable(test_class_basic test/main.cpp)
target_link_libraries(test_class_basic PRIVATE class_basic)
add_test(NAME class_basic_test COMMAND test_class_basic)
