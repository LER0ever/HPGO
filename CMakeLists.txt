cmake_minimum_required(VERSION 2.8)

project(HPGO)

if (APPLE)
    set(CMAKE_MACOSX_RPATH ON)
endif (APPLE)
set(CMAKE_CXX_STANDARD 17)


## Resource Files
# This will be our group of resource files
set(project_RESOURCE_FILES
        README.md
        .clang-format
        )

# Set properties for this group of files
set_source_files_properties(
        ${project_RESOURCE_FILES}
        PROPERTIES
        HEADER_FILE_ONLY TRUE # Since it's just a text file, it doesn't need compiled
        #MACOSX_PACKAGE_LOCATION Resource <- only do this if you need to copy the file!
)

# Bonus points unlocked :)
source_group(
        "Resources" FILES ${project_RESOURCE_FILES}
)


FIND_PACKAGE(PythonInterp)

if (PYTHONINTERP_FOUND)
    if (UNIX AND NOT APPLE)
        if (PYTHON_VERSION_MAJOR EQUAL 3)
            FIND_PACKAGE(Boost COMPONENTS python${PYTHON_VERSION_SUFFIX})
            FIND_PACKAGE(PythonInterp 3)
            FIND_PACKAGE(PythonLibs 3 REQUIRED)
        else ()
            FIND_PACKAGE(Boost COMPONENTS python)
            FIND_PACKAGE(PythonInterp)
            FIND_PACKAGE(PythonLibs REQUIRED)
        endif ()
    else ()
        if (PYTHON_VERSION_MAJOR EQUAL 3)
            FIND_PACKAGE(Boost COMPONENTS python${PYTHON_VERSION_MAJOR}${PYTHON_VERSION_MINOR})
            FIND_PACKAGE(PythonInterp 3)
            FIND_PACKAGE(PythonLibs 3 REQUIRED)
        else ()
            FIND_PACKAGE(Boost COMPONENTS python${PYTHON_VERSION_MAJOR}${PYTHON_VERSION_MINOR})
            FIND_PACKAGE(PythonInterp)
            FIND_PACKAGE(PythonLibs REQUIRED)
        endif ()
    endif ()
else ()
    message("Python not found")
endif ()

#PYTHON_ADD_MODULE(HPGO_PY
#        HPGO.h
#        HPGO.cc
#        orchestration.h
#        orchestration.cc
#        pipeline.h
#        pipeline.cc
#        syncpipeline.h
#        syncpipeline.cc
#        block.h
#        block.cc
#        duration.h
#        duration.cc
#        model.cc
#        model.h
#        )


# TESTS
add_executable("test_main"
        tests/catch.hpp
        tests/test_main.cc
        tests/test_helper.cc
        tests/test_PEC.cc
        tests/test_orchestrate_manual.cc
        tests/test_orchestrate_full.cc
        tests/test_pd.cc
        tests/test_graph.cc
        tests/test_model.cc
        tests/test_device.cc
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
        orchestration/orchestration.h
        orchestration/orchestration.cc
        parallelism/pipeline/pipeline.h
        parallelism/pipeline/pipeline.cc
        parallelism/pipeline/syncpipeline.h
        parallelism/pipeline/syncpipeline.cc
        parallelism/pipeline/block.h
        parallelism/pipeline/block.cc
        parallelism/pipeline/duration.h
        parallelism/pipeline/duration.cc
        parallelism/data-parallel/data-parallel.cc
        parallelism/data-parallel/data-parallel.h
        parallelism/split-concat/split-concat.cc
        parallelism/split-concat/split-concat.h
        model/model.cc
        model/model.h
        environment/device.cc
        environment/device.h
        environment/network.cc
        environment/network.h
        environment/nvlink.cc
        environment/nvlink.h
        utils/helper.cc
        utils/helper.h
        input/graph.cc
        input/graph.h
        ${project_RESOURCE_FILES}
        )

if (APPLE)
    target_include_directories(HPGO PUBLIC
            "/usr/local/include"
            "/usr/local/Cellar/python/3.7.4_1/Frameworks/Python.framework/Versions/3.7/include/python3.7m"
            )
    target_link_libraries("HPGO"
            ${Boost_LIBRARIES}
            "boost_python37"
            python3.7m
            dl
            )
    target_link_directories("HPGO" PUBLIC
            "/usr/local/Cellar/boost-python3/1.71.0/lib"
            "/usr/local/opt/python/Frameworks/Python.framework/Versions/3.7/lib/python3.7/config-3.7m-darwin"
            )
elseif (WIN32)
    target_include_directories(HPGO PUBLIC
            "E:/Code/C/vcpkg/installed/x86-windows/include"
            "E:/Code/C/vcpkg/installed/x86-windows/include/python3.7"
            )
    target_link_libraries("HPGO"
            ${Boost_LIBRARIES}
            "boost_python37-vc140-mt"
            python37
            )
    target_link_directories("HPGO" PUBLIC
            "E:/Code/C/vcpkg/installed/x86-windows/lib"
            )
endif ()

target_link_libraries(test_main HPGO)



