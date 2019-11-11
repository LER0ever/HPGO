
# 
# Find NodePPT
# 

find_program(NODEPPT nodeppt)
if(NOT NODEPPT)
    message(STATUS "Disabled generation of presentation (missing NodePPT).")
    return()
endif()


# 
# Target name
# 

set(target docs-presentation)
message(STATUS "Doc ${target}")


# 
# Input and output files
# 

set(source "${CMAKE_CURRENT_SOURCE_DIR}/HPGO.md")
set(html    "${CMAKE_CURRENT_BINARY_DIR}/HPGO-presentation/")


# 
# Create documentation
# 

# Invoke LaTeX
add_custom_command(
    OUTPUT            ${html}
    DEPENDS           ${source}
    WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
    COMMAND           ${NODEPPT} build \"${source}\"
    COMMENT           "Creating HPGO Presentation."
)

# Declare target
add_custom_target(${target} ALL DEPENDS ${html})
add_dependencies(docs ${target})


# 
# Deployment
# 

# PDF file
install(DIRECTORY ${html}
    DESTINATION "${INSTALL_DOC}"
    COMPONENT docs
)
