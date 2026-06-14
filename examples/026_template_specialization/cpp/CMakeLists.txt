cmake_minimum_required(VERSION 3.16)
project(hicc_usage_template_specialization CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB template_specialization_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(template_specialization STATIC ${template_specialization_SOURCES})
target_include_directories(template_specialization PUBLIC include)

enable_testing()
add_executable(test_template_specialization test/main.cpp)
target_link_libraries(test_template_specialization PRIVATE template_specialization)
add_test(NAME template_specialization_test COMMAND test_template_specialization)
