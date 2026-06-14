cmake_minimum_required(VERSION 3.16)
project(hicc_usage_friend_function CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB friend_function_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(friend_function STATIC ${friend_function_SOURCES})
target_include_directories(friend_function PUBLIC include)

enable_testing()
add_executable(test_friend_function test/main.cpp)
target_link_libraries(test_friend_function PRIVATE friend_function)
add_test(NAME friend_function_test COMMAND test_friend_function)
