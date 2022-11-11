cmake_minimum_required(VERSION 3.10)

project(demo VERSION 1.0.0 LANGUAGES CXX)
message("build type ${CMAKE_BUILD_TYPE}")

find_library(lib_demo_file 
    NAME demo_file
    PATHS ${PROJECT_SOURCE_DIR}/../target/release ${PROJECT_SOURCE_DIR}/../target/debug
    REQUIRED)
message("${lib_demo_file}")
add_executable(${PROJECT_NAME} main.cpp)
target_link_libraries(${PROJECT_NAME} ${lib_demo_file})
