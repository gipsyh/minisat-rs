cmake_minimum_required(VERSION 3.14)
project(bindings VERSION 1.0.0)

enable_language(C CXX)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fpermissive -Wno-reserved-user-defined-literal")

add_library(bindings binding.cpp)
target_include_directories(bindings PUBLIC ../minisat)

add_subdirectory(../minisat minisat)
target_link_libraries(bindings PUBLIC minisat-lib-static)

install(TARGETS bindings
        LIBRARY DESTINATION lib
        PUBLIC_HEADER DESTINATION include
        )