cmake_minimum_required(VERSION 2.8)

project(HPGO)

if (APPLE)
    set(CMAKE_MACOSX_RPATH ON)
endif (APPLE)
set (CMAKE_CXX_STANDARD 17)

#add_library("HPGO" SHARED
#    "block.cc"
#    "block.h"
#    "pipeline.cc"
#    "pipeline.h"
#    "duration.cc"
#    "duration.h"
#    "syncpipeline.cc"
#    "syncpipeline.h"
#)


FIND_PACKAGE(PythonInterp)

if (PYTHONINTERP_FOUND)
    if (UNIX AND NOT APPLE)
        if (PYTHON_VERSION_MAJOR EQUAL 3)
            FIND_PACKAGE(Boost COMPONENTS python${PYTHON_VERSION_SUFFIX})
            FIND_PACKAGE(PythonInterp 3)
            FIND_PACKAGE(PythonLibs 3 REQUIRED)
        else()
            FIND_PACKAGE(Boost COMPONENTS python)
            FIND_PACKAGE(PythonInterp)
            FIND_PACKAGE(PythonLibs REQUIRED)
        endif()
    else()
        if (PYTHON_VERSION_MAJOR EQUAL 3)
            FIND_PACKAGE(Boost COMPONENTS python${PYTHON_VERSION_MAJOR}${PYTHON_VERSION_MINOR})
            FIND_PACKAGE(PythonInterp 3)
            FIND_PACKAGE(PythonLibs 3 REQUIRED)
        else()
            FIND_PACKAGE(Boost COMPONENTS python${PYTHON_VERSION_MAJOR}${PYTHON_VERSION_MINOR})
            FIND_PACKAGE(PythonInterp)
            FIND_PACKAGE(PythonLibs REQUIRED)
        endif()
    endif()
else()
    message("Python not found")
endif()

PYTHON_ADD_MODULE(HPGO
        HPGO.h
        HPGO.cc
        orchestration.h
        orchestration.cc
        "pipeline.h"
        "pipeline.cc"
        "syncpipeline.h"
        "syncpipeline.cc"
        "block.h"
        "block.cc"
        "duration.h"
        "duration.cc"
        "model.cc"
        "model.h"
        )
target_include_directories(HPGO PUBLIC
    "/usr/local/include"
    "/usr/local/Cellar/python/3.7.4_1/Frameworks/Python.framework/Versions/3.7/include/python3.7m"
)

target_link_directories("HPGO" PUBLIC
        "/usr/local/Cellar/boost-python3/1.71.0/lib"
        "/usr/local/opt/python/Frameworks/Python.framework/Versions/3.7/lib/python3.7/config-3.7m-darwin")
# message(${Boost_LIBRARIES})
target_link_libraries("HPGO"
    ${Boost_LIBRARIES}
    "boost_python37"
    python3.7m
    dl
)


# TESTS
add_executable("test_PEC"
        "test-PEC.cc"
        orchestration.h
        orchestration.cc
        "pipeline.h"
        "pipeline.cc"
        "syncpipeline.h"
        "syncpipeline.cc"
        "block.h"
        "block.cc"
        "duration.h"
        "duration.cc"
        "model.cc"
        "model.h"
        )
add_executable("test_Orchestrate"
        "test-orchestrate.cc"
        orchestration.h
        orchestration.cc
        "pipeline.h"
        "pipeline.cc"
        "syncpipeline.h"
        "syncpipeline.cc"
        "block.h"
        "block.cc"
        "duration.h"
        "duration.cc"
        "model.cc"
        "model.h"
        )

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-error=sign-conversion -Wconversion -Wno-sign-conversion")


# add_executable(${PROJECT_NAME} "main.cc")

