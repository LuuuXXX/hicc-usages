cmake_minimum_required(VERSION 3.16)
project(hicc_usage_typeid_rtti CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB typeid_rtti_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(typeid_rtti STATIC ${typeid_rtti_SOURCES})
target_include_directories(typeid_rtti PUBLIC include)

enable_testing()
add_executable(test_typeid_rtti test/main.cpp)
target_link_libraries(test_typeid_rtti PRIVATE typeid_rtti)
add_test(NAME typeid_rtti_test COMMAND test_typeid_rtti)
