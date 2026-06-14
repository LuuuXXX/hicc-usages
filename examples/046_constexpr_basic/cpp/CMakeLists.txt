cmake_minimum_required(VERSION 3.16)
project(hicc_usage_constexpr_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB constexpr_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(constexpr_basic STATIC ${constexpr_basic_SOURCES})
target_include_directories(constexpr_basic PUBLIC include)

enable_testing()
add_executable(test_constexpr_basic test/main.cpp)
target_link_libraries(test_constexpr_basic PRIVATE constexpr_basic)
add_test(NAME constexpr_basic_test COMMAND test_constexpr_basic)
