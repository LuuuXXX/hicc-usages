cmake_minimum_required(VERSION 3.16)
project(hicc_usage_enum_class CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB enum_class_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(enum_class STATIC ${enum_class_SOURCES})
target_include_directories(enum_class PUBLIC include)

enable_testing()
add_executable(test_enum_class test/main.cpp)
target_link_libraries(test_enum_class PRIVATE enum_class)
add_test(NAME enum_class_test COMMAND test_enum_class)
