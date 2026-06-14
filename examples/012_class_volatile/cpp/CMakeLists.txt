cmake_minimum_required(VERSION 3.16)
project(hicc_usage_class_volatile CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB class_volatile_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(class_volatile STATIC ${class_volatile_SOURCES})
target_include_directories(class_volatile PUBLIC include)

enable_testing()
add_executable(test_class_volatile test/main.cpp)
target_link_libraries(test_class_volatile PRIVATE class_volatile)
add_test(NAME class_volatile_test COMMAND test_class_volatile)
