cmake_minimum_required(VERSION 3.16)
project(hicc_usage_inheritance_single CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB inheritance_single_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(inheritance_single STATIC ${inheritance_single_SOURCES})
target_include_directories(inheritance_single PUBLIC include)

enable_testing()
add_executable(test_inheritance_single test/main.cpp)
target_link_libraries(test_inheritance_single PRIVATE inheritance_single)
add_test(NAME inheritance_single_test COMMAND test_inheritance_single)
