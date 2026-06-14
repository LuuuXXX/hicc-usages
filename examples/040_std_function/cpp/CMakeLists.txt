cmake_minimum_required(VERSION 3.16)
project(hicc_usage_std_function CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB std_function_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(std_function STATIC ${std_function_SOURCES})
target_include_directories(std_function PUBLIC include)

enable_testing()
add_executable(test_std_function test/main.cpp)
target_link_libraries(test_std_function PRIVATE std_function)
add_test(NAME std_function_test COMMAND test_std_function)
