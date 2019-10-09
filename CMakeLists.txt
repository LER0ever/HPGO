cmake_minimum_required(VERSION 2.8)

project(HPGO)


add_executable("test_PEC"
    "test-PEC.cc"
    "pipeline.h"
    "pipeline.cc"
    "syncpipeline.h"
    "syncpipeline.cc"
    "block.h"
    "block.cc"
    "duration.h"
    "duration.cc"
)

target_compile_options("test_PEC" PRIVATE -Wall -Wextra -Wno-error=sign-conversion -Wconversion
    -Wno-sign-conversion -Wno-unused-parameter -Wno-sign-compare)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-error=sign-conversion -Wconversion -Wno-sign-conversion")


add_executable(${PROJECT_NAME} "main.cc")

