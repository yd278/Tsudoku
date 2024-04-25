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
    ${CMAKE_CURRENT_SOURCE_DIR}/engine/includes  # Include your project headers
)

# Source files
set(SOURCE_FILES
    engine/src/engineAPI.cpp
    engine/src/Grid.cpp
    engine/src/util.cpp;

    #Solvers
    engine/src/solvers/DLX.cpp
    engine/src/solvers/simpleSolvers.cpp
    engine/src/solvers/subsetFinder.cpp
    engine/src/solvers/wings.cpp
    engine/src/solvers/fish.cpp
    engine/src/solvers/singleDigit.cpp
    engine/src/solvers/uniqueness.cpp
    engine/src/solvers/coloring.cpp
)

# Header files are listed for better project management in some IDEs
set(HEADER_FILES
    engine/includes/Grid.h
    engine/includes/util.h
    engine/includes/util_const.h

    #solvers
    engine/includes/solvers/DLX.h
    engine/includes/solvers/simpleSolvers.h
    engine/includes/solvers/subsetFinder.h
    engine/includes/solvers/wings.h
    engine/includes/solvers/fish.h
    engine/includes/solvers/singleDigit.h
    engine/includes/solvers/uniqueness.h
    engine/includes/solvers/coloring.h
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
    tests/solvers/test_subset.cpp
    tests/solvers/test_wings.cpp
    tests/solvers/test_fish.cpp
    tests/solvers/test_singleDigit.cpp
    tests/solvers/test_uniqueness.cpp
    tests/solvers/test_coloring.cpp
)

add_executable(runTests ${TEST_SOURCES})
target_link_libraries(runTests gtest gtest_main ${PROJECT_NAME})

if(MSVC AND CMAKE_JS_NODELIB_DEF AND CMAKE_JS_NODELIB_TARGET)
    execute_process(COMMAND ${CMAKE_AR} /def:${CMAKE_JS_NODELIB_DEF} /out:${CMAKE_JS_NODELIB_TARGET} ${CMAKE_STATIC_LINKER_FLAGS})
endif()
