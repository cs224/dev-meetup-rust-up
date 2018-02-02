cmake_minimum_required(VERSION 3.9)
project(cc-factorial VERSION 0.1.0 DESCRIPTION "cc-factorial")

set(CMAKE_CXX_STANDARD 14)

add_executable(cc-factorial main.cpp cc_factorial.h cc_factorial.cpp)

# https://stackoverflow.com/questions/17511496/how-to-create-a-shared-library-with-cmake

add_library(cc_factorial SHARED cc_factorial.h cc_factorial.cpp)
set_target_properties(cc_factorial PROPERTIES
        VERSION ${PROJECT_VERSION}
        SOVERSION 1
        PUBLIC_HEADER cc_factorial.h)