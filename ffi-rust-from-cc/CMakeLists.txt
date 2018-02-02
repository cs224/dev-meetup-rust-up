cmake_minimum_required(VERSION 3.9)
project(ffi_rust_from_cc)

set(CMAKE_CXX_STANDARD 14)

add_library(cc_factorial SHARED IMPORTED)
#SET_PROPERTY(TARGET cc_factorial PROPERTY IMPORTED_LOCATION ../../cc-factorial/cmake-build-debug/libcc_factorial.so)
SET_PROPERTY(TARGET cc_factorial PROPERTY IMPORTED_LOCATION ../../rust-factorial/target/debug/librust_factorial.so)


add_executable(ffi_rust_from_cc main.cpp)

target_link_libraries(ffi_rust_from_cc cc_factorial)