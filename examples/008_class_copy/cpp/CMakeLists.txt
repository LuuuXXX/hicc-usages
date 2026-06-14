cmake_minimum_required(VERSION 3.16)
project(hicc_usage_class_copy CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB class_copy_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(class_copy STATIC ${class_copy_SOURCES})
target_include_directories(class_copy PUBLIC include)

enable_testing()
add_executable(test_class_copy test/main.cpp)
target_link_libraries(test_class_copy PRIVATE class_copy)
add_test(NAME class_copy_test COMMAND test_class_copy)
