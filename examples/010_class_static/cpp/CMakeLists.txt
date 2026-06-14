cmake_minimum_required(VERSION 3.16)
project(hicc_usage_class_static CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB class_static_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(class_static STATIC ${class_static_SOURCES})
target_include_directories(class_static PUBLIC include)

enable_testing()
add_executable(test_class_static test/main.cpp)
target_link_libraries(test_class_static PRIVATE class_static)
add_test(NAME class_static_test COMMAND test_class_static)
