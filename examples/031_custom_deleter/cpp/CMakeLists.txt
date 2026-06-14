cmake_minimum_required(VERSION 3.16)
project(hicc_usage_custom_deleter CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB custom_deleter_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(custom_deleter STATIC ${custom_deleter_SOURCES})
target_include_directories(custom_deleter PUBLIC include)

enable_testing()
add_executable(test_custom_deleter test/main.cpp)
target_link_libraries(test_custom_deleter PRIVATE custom_deleter)
add_test(NAME custom_deleter_test COMMAND test_custom_deleter)
