cmake_minimum_required(VERSION 3.16)
project(hicc_usage_template_instantiation CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB template_instantiation_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(template_instantiation STATIC ${template_instantiation_SOURCES})
target_include_directories(template_instantiation PUBLIC include)

enable_testing()
add_executable(test_template_instantiation test/main.cpp)
target_link_libraries(test_template_instantiation PRIVATE template_instantiation)
add_test(NAME template_instantiation_test COMMAND test_template_instantiation)
