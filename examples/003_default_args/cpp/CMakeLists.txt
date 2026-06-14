cmake_minimum_required(VERSION 3.16)
project(hicc_usage_default_args CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB default_args_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(default_args STATIC ${default_args_SOURCES})
target_include_directories(default_args PUBLIC include)

enable_testing()
add_executable(test_default_args test/main.cpp)
target_link_libraries(test_default_args PRIVATE default_args)
add_test(NAME default_args_test COMMAND test_default_args)
