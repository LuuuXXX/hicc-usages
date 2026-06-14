cmake_minimum_required(VERSION 3.16)
project(hicc_usage_explicit_ctor CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB explicit_ctor_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(explicit_ctor STATIC ${explicit_ctor_SOURCES})
target_include_directories(explicit_ctor PUBLIC include)

enable_testing()
add_executable(test_explicit_ctor test/main.cpp)
target_link_libraries(test_explicit_ctor PRIVATE explicit_ctor)
add_test(NAME explicit_ctor_test COMMAND test_explicit_ctor)
