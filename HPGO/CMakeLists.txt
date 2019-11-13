
# 
# External dependencies
# 

# find_package(THIRDPARTY REQUIRED)
SET(Python_ADDITIONAL_VERSIONS 3 3.6 3.7)
SET(PYTHON_LIBRARY /usr/local/opt/python/Frameworks/Python.framework/Versions/3.7/lib/libpython3.7.dylib)
SET(PYTHON_INCLUDE_DIR /usr/local/Cellar/python/3.7.4_1/Frameworks/Python.framework/Versions/3.7/include/python3.7m)
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

# 
# Library name and options
# 

# Target name
set(target HPGO)

# Exit here if required dependencies are not met
message(STATUS "Lib ${target}")

# Set API export file and macro
string(MAKE_C_IDENTIFIER ${target} target_id)
string(TOUPPER ${target_id} target_id)
set(feature_file         "include/${target}/${target}_features.h")
set(export_file          "include/${target}/${target}_export.h")
set(template_export_file "include/${target}/${target}_api.h")
set(export_macro         "${target_id}_API")

# 
# Sources
# 

set(include_path "${CMAKE_CURRENT_SOURCE_DIR}/include") # /${target}
set(source_path  "${CMAKE_CURRENT_SOURCE_DIR}/source")
set(src "${CMAKE_CURRENT_SOURCE_DIR}")

set(headers
    ${src}/orchestration/orchestration.h
    ${src}/parallelism/pipeline/pipeline.h
    ${src}/parallelism/pipeline/syncpipeline.h
    ${src}/parallelism/pipeline/block.h
    ${src}/parallelism/pipeline/duration.h
    ${src}/parallelism/data-parallel/data-parallel.h
    ${src}/parallelism/split-concat/split-concat.h
    ${src}/model/model.h
    ${src}/environment/device.h
    ${src}/environment/network.h
    ${src}/environment/nvlink.h
    ${src}/utils/helper.h
    ${src}/input/graph.h
)

set(sources
    ${src}/orchestration/orchestration-hierarchical.cc
    ${src}/orchestration/orchestration.cc
    ${src}/parallelism/pipeline/pipeline.cc
    ${src}/parallelism/pipeline/syncpipeline.cc
    ${src}/parallelism/pipeline/block.cc
    ${src}/parallelism/pipeline/duration.cc
    ${src}/parallelism/data-parallel/data-parallel.cc
    ${src}/parallelism/split-concat/split-concat.cc
    ${src}/model/model.cc
    ${src}/environment/device.cc
    ${src}/environment/network.cc
    ${src}/environment/nvlink.cc
    ${src}/utils/helper.cc
    ${src}/input/graph.cc
)

set(api_files
	${src}/HPGO.h
	${src}/HPGO.cc
)

# Group source files
set(api_group "Header Files (API)")
set(header_group "Header Files")
set(source_group "Source Files")
source_group_by_path(${src} "\\\\.h$|\\\\.hpp$" 
    ${header_group} ${headers})
source_group_by_path(${src}  "\\\\.cpp$|\\\\.c$|\\\\.cc$|\\\\.h$|\\\\.hpp$" 
    ${source_group} ${sources})
source_group_by_path(${src} "\\\\.cpp$|\\\\.c$|\\\\.cc$|\\\\.h$|\\\\.hpp$"
	${api_group} ${api_files})

# 
# Create library
# 

# Build library
add_library(${target}
    ${sources}
    ${headers}
	${api_files}
)

# Create namespaced alias
add_library(${META_PROJECT_NAME}::${target} ALIAS ${target})

# Export library for downstream projects
export(TARGETS ${target} NAMESPACE ${META_PROJECT_NAME}:: FILE ${PROJECT_BINARY_DIR}/cmake/${target}/${target}-export.cmake)

# Create feature detection header
# Compilers: https://cmake.org/cmake/help/v3.1/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID
# Feature: https://cmake.org/cmake/help/v3.1/prop_gbl/CMAKE_CXX_KNOWN_FEATURES.html

# Check for availability of module; use pre-generated version if not found
if (WriterCompilerDetectionHeaderFound)
    write_compiler_detection_header(
        FILE ${feature_file}
        PREFIX ${target_id}
        COMPILERS AppleClang Clang GNU MSVC
        FEATURES cxx_alignas cxx_alignof cxx_constexpr cxx_final cxx_noexcept cxx_nullptr cxx_sizeof_member cxx_thread_local
        VERSION 3.2
    )
else()
    file(
        COPY ${PROJECT_SOURCE_DIR}/codegeneration/${target}_features.h
        DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/include/${target}
        USE_SOURCE_PERMISSIONS
    )
endif()

# Create API export header
generate_export_header(${target}
    EXPORT_FILE_NAME  ${export_file}
    EXPORT_MACRO_NAME ${export_macro}
)

generate_template_export_header(${target}
    ${target_id}
    ${template_export_file}
)


# 
# Project options
# 

set_target_properties(${target}
    PROPERTIES
    ${DEFAULT_PROJECT_OPTIONS}
    FOLDER "${IDE_FOLDER}"
    VERSION ${META_VERSION}
    SOVERSION ${META_VERSION_MAJOR}
)


# 
# Include directories
# 

target_include_directories(${target}
    PRIVATE
    ${PROJECT_BINARY_DIR}/source/include
	${PROJECT_BINARY_DIR}/include
    ${PROJECT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_BINARY_DIR}/include

    PUBLIC
    ${DEFAULT_INCLUDE_DIRECTORIES}
	${Boost_INCLUDE_DIRS}
	${PYTHON_INCLUDE_DIRS}

    INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}/include>
    $<INSTALL_INTERFACE:include>
)


# 
# Libraries
# 

target_link_libraries(${target}
    PRIVATE

    PUBLIC
    ${DEFAULT_LIBRARIES}
	${Boost_LIBRARIES}
	${PYTHON_LIBRARIES}
	${PYTHON_LIBRARY}

    INTERFACE
)


# 
# Compile definitions
# 

target_compile_definitions(${target}
    PRIVATE

    PUBLIC
    $<$<NOT:$<BOOL:${BUILD_SHARED_LIBS}>>:${target_id}_STATIC_DEFINE>
    ${DEFAULT_COMPILE_DEFINITIONS}

    INTERFACE
)


# 
# Compile options
# 

target_compile_options(${target}
    PRIVATE

    PUBLIC
    ${DEFAULT_COMPILE_OPTIONS}

    INTERFACE
)


# 
# Linker options
# 

target_link_libraries(${target}
    PRIVATE

    PUBLIC
    ${DEFAULT_LINKER_OPTIONS}
	${Boost_LIBRARIES}
	${PYTHON_LIBRARIES}

    INTERFACE
)


#
# Target Health
#

perform_health_checks(
    ${target}
    ${sources}
    ${headers}
	${api_files}
)


# 
# Deployment
# 

# Library
install(TARGETS ${target}
    EXPORT  "${target}-export"            COMPONENT dev
    RUNTIME DESTINATION ${INSTALL_BIN}    COMPONENT runtime
    LIBRARY DESTINATION ${INSTALL_SHARED} COMPONENT runtime
    ARCHIVE DESTINATION ${INSTALL_LIB}    COMPONENT dev
)

# Header files
install(FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/${target}.h DESTINATION ${INSTALL_INCLUDE}
    COMPONENT dev
)

install(FILES
    ${headers} DESTINATION ${INSTALL_INCLUDE}
    COMPONENT dev
)

# Generated header files
install(DIRECTORY
    ${CMAKE_CURRENT_BINARY_DIR}/include/${target} DESTINATION ${INSTALL_INCLUDE}
    COMPONENT dev
)

# CMake config
install(EXPORT ${target}-export
    NAMESPACE   ${META_PROJECT_NAME}::
    DESTINATION ${INSTALL_CMAKE}/${target}
    COMPONENT   dev
)
