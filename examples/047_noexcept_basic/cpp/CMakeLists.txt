cmake_minimum_required(VERSION 3.16)
project(hicc_usage_noexcept_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB noexcept_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(noexcept_basic STATIC ${noexcept_basic_SOURCES})
target_include_directories(noexcept_basic PUBLIC include)

enable_testing()
add_executable(test_noexcept_basic test/main.cpp)
target_link_libraries(test_noexcept_basic PRIVATE noexcept_basic)
add_test(NAME noexcept_basic_test COMMAND test_noexcept_basic)
