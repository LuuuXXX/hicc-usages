cmake_minimum_required(VERSION 3.16)
project(hicc_usage_variadic_functions CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB variadic_functions_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(variadic_functions STATIC ${variadic_functions_SOURCES})
target_include_directories(variadic_functions PUBLIC include)

enable_testing()
add_executable(test_variadic_functions test/main.cpp)
target_link_libraries(test_variadic_functions PRIVATE variadic_functions)
add_test(NAME variadic_functions_test COMMAND test_variadic_functions)
