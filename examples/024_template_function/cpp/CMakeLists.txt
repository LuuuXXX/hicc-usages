cmake_minimum_required(VERSION 3.16)
project(hicc_usage_template_function CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB template_function_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(template_function STATIC ${template_function_SOURCES})
target_include_directories(template_function PUBLIC include)

enable_testing()
add_executable(test_template_function test/main.cpp)
target_link_libraries(test_template_function PRIVATE template_function)
add_test(NAME template_function_test COMMAND test_template_function)
