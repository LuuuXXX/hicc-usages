cmake_minimum_required(VERSION 3.16)
project(hicc_usage_exception_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB exception_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(exception_basic STATIC ${exception_basic_SOURCES})
target_include_directories(exception_basic PUBLIC include)

enable_testing()
add_executable(test_exception_basic test/main.cpp)
target_link_libraries(test_exception_basic PRIVATE exception_basic)
add_test(NAME exception_basic_test COMMAND test_exception_basic)
