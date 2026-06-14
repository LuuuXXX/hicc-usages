cmake_minimum_required(VERSION 3.16)
project(hicc_usage_virtual_override CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB virtual_override_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(virtual_override STATIC ${virtual_override_SOURCES})
target_include_directories(virtual_override PUBLIC include)

enable_testing()
add_executable(test_virtual_override test/main.cpp)
target_link_libraries(test_virtual_override PRIVATE virtual_override)
add_test(NAME virtual_override_test COMMAND test_virtual_override)
