cmake_minimum_required(VERSION 3.15)
project(tsudoku-engine)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
add_definitions(-DNAPI_VERSION=8)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

# Include directories
include_directories(
    ${CMAKE_JS_INC}                   # Include node-addon headers
    ${CMAKE_CURRENT_SOURCE_DIR}/src/engine  # Include your project headers
)

# Source files
set(SOURCE_FILES
    src/engine/engine.cpp
    src/engine/engineAPI.cpp
    src/engine/Grid.cpp
    src/engine/util.cpp;

    #Solvers
    src/engine/solvers/DLX.cpp
    src/engine/solvers/simpleSolvers.cpp
)

# Header files are listed for better project management in some IDEs
set(HEADER_FILES
    src/engine/engine.h
    src/engine/Grid.h
    src/engine/util.h

    #solvers
    src/engine/solvers/DLX.h
    src/engine/solvers/simpleSolvers.h
)

add_library(${PROJECT_NAME} SHARED ${SOURCE_FILES} ${HEADER_FILES} ${CMAKE_JS_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "" SUFFIX ".node")
target_link_libraries(${PROJECT_NAME} ${CMAKE_JS_LIB})

enable_testing()
add_subdirectory(extern/googletest)
include_directories(${gtest_SOURCE_DIR}/include ${gtest_SOURCE_DIR})

set(TEST_SOURCES
    tests/test_main.cpp
    tests/test_Grid.cpp
    tests/solvers/test_DLX.cpp
    tests/solvers/test_solvers.cpp
)

add_executable(runTests ${TEST_SOURCES})
target_link_libraries(runTests gtest gtest_main ${PROJECT_NAME})

if(MSVC AND CMAKE_JS_NODELIB_DEF AND CMAKE_JS_NODELIB_TARGET)
    execute_process(COMMAND ${CMAKE_AR} /def:${CMAKE_JS_NODELIB_DEF} /out:${CMAKE_JS_NODELIB_TARGET} ${CMAKE_STATIC_LINKER_FLAGS})
endif()
