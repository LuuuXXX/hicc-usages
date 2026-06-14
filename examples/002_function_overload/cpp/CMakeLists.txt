cmake_minimum_required(VERSION 3.16)
project(hicc_usage_function_overload CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB function_overload_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(function_overload STATIC ${function_overload_SOURCES})
target_include_directories(function_overload PUBLIC include)

enable_testing()
add_executable(test_function_overload test/main.cpp)
target_link_libraries(test_function_overload PRIVATE function_overload)
add_test(NAME function_overload_test COMMAND test_function_overload)
