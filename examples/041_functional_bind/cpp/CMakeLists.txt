cmake_minimum_required(VERSION 3.16)
project(hicc_usage_functional_bind CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB functional_bind_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(functional_bind STATIC ${functional_bind_SOURCES})
target_include_directories(functional_bind PUBLIC include)

enable_testing()
add_executable(test_functional_bind test/main.cpp)
target_link_libraries(test_functional_bind PRIVATE functional_bind)
add_test(NAME functional_bind_test COMMAND test_functional_bind)
