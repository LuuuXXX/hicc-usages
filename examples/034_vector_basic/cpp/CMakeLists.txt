cmake_minimum_required(VERSION 3.16)
project(hicc_usage_vector_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB vector_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(vector_basic STATIC ${vector_basic_SOURCES})
target_include_directories(vector_basic PUBLIC include)

enable_testing()
add_executable(test_vector_basic test/main.cpp)
target_link_libraries(test_vector_basic PRIVATE vector_basic)
add_test(NAME vector_basic_test COMMAND test_vector_basic)
