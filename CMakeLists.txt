cmake_minimum_required(VERSION 2.8)

project(HPGO)

if (APPLE)
    set(CMAKE_MACOSX_RPATH ON)
endif (APPLE)
set (CMAKE_CXX_STANDARD 17)




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

PYTHON_ADD_MODULE(HPGO_PY
        HPGO.h
        HPGO.cc
        orchestration.h
        orchestration.cc
        pipeline.h
        pipeline.cc
        syncpipeline.h
        syncpipeline.cc
        block.h
        block.cc
        duration.h
        duration.cc
        model.cc
        model.h
        )

if(APPLE)
    target_include_directories(HPGO_PY PUBLIC
        "/usr/local/include"
        "/usr/local/Cellar/python/3.7.4_1/Frameworks/Python.framework/Versions/3.7/include/python3.7m"
    )
    target_link_libraries("HPGO_PY"
        ${Boost_LIBRARIES}
        "boost_python37"
        python3.7m
        dl
    )
    target_link_directories("HPGO_PY" PUBLIC
        "/usr/local/Cellar/boost-python3/1.71.0/lib"
        "/usr/local/opt/python/Frameworks/Python.framework/Versions/3.7/lib/python3.7/config-3.7m-darwin"
    )
elseif(WIN32)
    target_include_directories(HPGO_PY PUBLIC
        "E:/Code/C/vcpkg/installed/x86-windows/include"
        "E:/Code/C/vcpkg/installed/x86-windows/include/python3.7"
    )
    target_link_libraries("HPGO_PY"
        ${Boost_LIBRARIES}
        "boost_python37-vc140-mt"
        python37
    )
    target_link_directories("HPGO_PY" PUBLIC
        "E:/Code/C/vcpkg/installed/x86-windows/lib"
    )
endif()


# TESTS
add_executable("test_main"
    tests/catch.hpp
    tests/test_main.cc
    tests/test_helper.cc
    tests/test_PEC.cc
    tests/test_orchestrate.cc
#    orchestration.h
#    orchestration.cc
#    pipeline.h
#    pipeline.cc
#    syncpipeline.h
#    syncpipeline.cc
#    block.h
#    block.cc
#    duration.h
#    duration.cc
#    model.cc
#    model.h
#    helper.cc
#    helper.h
)

add_library(HPGO SHARED
    orchestration.h
    orchestration.cc
    pipeline.h
    pipeline.cc
    syncpipeline.h
    syncpipeline.cc
    block.h
    block.cc
    duration.h
    duration.cc
    model.cc
    model.h
    helper.cc
    helper.h
)

target_link_libraries(test_main HPGO)

# add_executable(${PROJECT_NAME} "main.cc")

