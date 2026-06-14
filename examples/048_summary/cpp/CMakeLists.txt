cmake_minimum_required(VERSION 3.16)
project(hicc_usage_summary CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB summary_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(summary STATIC ${summary_SOURCES})
target_include_directories(summary PUBLIC include)

enable_testing()
add_executable(test_summary test/main.cpp)
target_link_libraries(test_summary PRIVATE summary)
add_test(NAME summary_test COMMAND test_summary)
