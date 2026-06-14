cmake_minimum_required(VERSION 3.16)
project(hicc_usage_inheritance_multiple CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB inheritance_multiple_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(inheritance_multiple STATIC ${inheritance_multiple_SOURCES})
target_include_directories(inheritance_multiple PUBLIC include)

enable_testing()
add_executable(test_inheritance_multiple test/main.cpp)
target_link_libraries(test_inheritance_multiple PRIVATE inheritance_multiple)
add_test(NAME inheritance_multiple_test COMMAND test_inheritance_multiple)
