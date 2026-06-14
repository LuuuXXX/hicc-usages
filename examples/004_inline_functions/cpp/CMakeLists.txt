cmake_minimum_required(VERSION 3.16)
project(hicc_usage_inline_functions CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB inline_functions_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(inline_functions STATIC ${inline_functions_SOURCES})
target_include_directories(inline_functions PUBLIC include)

enable_testing()
add_executable(test_inline_functions test/main.cpp)
target_link_libraries(test_inline_functions PRIVATE inline_functions)
add_test(NAME inline_functions_test COMMAND test_inline_functions)
