cmake_minimum_required(VERSION 3.16)
project(hicc_usage_shared_ptr CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB shared_ptr_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(shared_ptr STATIC ${shared_ptr_SOURCES})
target_include_directories(shared_ptr PUBLIC include)

enable_testing()
add_executable(test_shared_ptr test/main.cpp)
target_link_libraries(test_shared_ptr PRIVATE shared_ptr)
add_test(NAME shared_ptr_test COMMAND test_shared_ptr)
