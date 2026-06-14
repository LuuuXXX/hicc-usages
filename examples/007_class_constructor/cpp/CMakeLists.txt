cmake_minimum_required(VERSION 3.16)
project(hicc_usage_class_constructor CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB class_constructor_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(class_constructor STATIC ${class_constructor_SOURCES})
target_include_directories(class_constructor PUBLIC include)

enable_testing()
add_executable(test_class_constructor test/main.cpp)
target_link_libraries(test_class_constructor PRIVATE class_constructor)
add_test(NAME class_constructor_test COMMAND test_class_constructor)
