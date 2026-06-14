cmake_minimum_required(VERSION 3.16)
project(hicc_usage_mutable_member CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB mutable_member_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(mutable_member STATIC ${mutable_member_SOURCES})
target_include_directories(mutable_member PUBLIC include)

enable_testing()
add_executable(test_mutable_member test/main.cpp)
target_link_libraries(test_mutable_member PRIVATE mutable_member)
add_test(NAME mutable_member_test COMMAND test_mutable_member)
