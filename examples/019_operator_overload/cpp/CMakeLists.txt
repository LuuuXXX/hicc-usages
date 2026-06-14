cmake_minimum_required(VERSION 3.16)
project(hicc_usage_operator_overload CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB operator_overload_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(operator_overload STATIC ${operator_overload_SOURCES})
target_include_directories(operator_overload PUBLIC include)

enable_testing()
add_executable(test_operator_overload test/main.cpp)
target_link_libraries(test_operator_overload PRIVATE operator_overload)
add_test(NAME operator_overload_test COMMAND test_operator_overload)
