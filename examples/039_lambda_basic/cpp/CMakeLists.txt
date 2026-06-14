cmake_minimum_required(VERSION 3.16)
project(hicc_usage_lambda_basic CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB lambda_basic_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(lambda_basic STATIC ${lambda_basic_SOURCES})
target_include_directories(lambda_basic PUBLIC include)

enable_testing()
add_executable(test_lambda_basic test/main.cpp)
target_link_libraries(test_lambda_basic PRIVATE lambda_basic)
add_test(NAME lambda_basic_test COMMAND test_lambda_basic)
