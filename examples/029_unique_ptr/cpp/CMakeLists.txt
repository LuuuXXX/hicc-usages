cmake_minimum_required(VERSION 3.16)
project(hicc_usage_unique_ptr CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB unique_ptr_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(unique_ptr STATIC ${unique_ptr_SOURCES})
target_include_directories(unique_ptr PUBLIC include)

enable_testing()
add_executable(test_unique_ptr test/main.cpp)
target_link_libraries(test_unique_ptr PRIVATE unique_ptr)
add_test(NAME unique_ptr_test COMMAND test_unique_ptr)
