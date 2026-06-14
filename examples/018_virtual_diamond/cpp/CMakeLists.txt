cmake_minimum_required(VERSION 3.16)
project(hicc_usage_virtual_diamond CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB virtual_diamond_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(virtual_diamond STATIC ${virtual_diamond_SOURCES})
target_include_directories(virtual_diamond PUBLIC include)

enable_testing()
add_executable(test_virtual_diamond test/main.cpp)
target_link_libraries(test_virtual_diamond PRIVATE virtual_diamond)
add_test(NAME virtual_diamond_test COMMAND test_virtual_diamond)
