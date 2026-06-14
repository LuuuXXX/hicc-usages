cmake_minimum_required(VERSION 3.16)
project(hicc_usage_placement_new CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB placement_new_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(placement_new STATIC ${placement_new_SOURCES})
target_include_directories(placement_new PUBLIC include)

enable_testing()
add_executable(test_placement_new test/main.cpp)
target_link_libraries(test_placement_new PRIVATE placement_new)
add_test(NAME placement_new_test COMMAND test_placement_new)
