cmake_minimum_required(VERSION 3.16)
project(hicc_usage_variadic_template CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB variadic_template_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(variadic_template STATIC ${variadic_template_SOURCES})
target_include_directories(variadic_template PUBLIC include)

enable_testing()
add_executable(test_variadic_template test/main.cpp)
target_link_libraries(test_variadic_template PRIVATE variadic_template)
add_test(NAME variadic_template_test COMMAND test_variadic_template)
