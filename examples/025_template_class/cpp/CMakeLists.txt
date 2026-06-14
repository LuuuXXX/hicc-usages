cmake_minimum_required(VERSION 3.16)
project(hicc_usage_template_class CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB template_class_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(template_class STATIC ${template_class_SOURCES})
target_include_directories(template_class PUBLIC include)

enable_testing()
add_executable(test_template_class test/main.cpp)
target_link_libraries(test_template_class PRIVATE template_class)
add_test(NAME template_class_test COMMAND test_template_class)
