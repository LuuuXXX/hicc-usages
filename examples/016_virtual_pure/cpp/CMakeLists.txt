cmake_minimum_required(VERSION 3.16)
project(hicc_usage_virtual_pure CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB virtual_pure_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(virtual_pure STATIC ${virtual_pure_SOURCES})
target_include_directories(virtual_pure PUBLIC include)

enable_testing()
add_executable(test_virtual_pure test/main.cpp)
target_link_libraries(test_virtual_pure PRIVATE virtual_pure)
add_test(NAME virtual_pure_test COMMAND test_virtual_pure)
