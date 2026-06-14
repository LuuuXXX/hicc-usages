cmake_minimum_required(VERSION 3.16)
project(hicc_usage_raii_pattern CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB raii_pattern_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(raii_pattern STATIC ${raii_pattern_SOURCES})
target_include_directories(raii_pattern PUBLIC include)

enable_testing()
add_executable(test_raii_pattern test/main.cpp)
target_link_libraries(test_raii_pattern PRIVATE raii_pattern)
add_test(NAME raii_pattern_test COMMAND test_raii_pattern)
