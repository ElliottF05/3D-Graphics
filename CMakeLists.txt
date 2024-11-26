cmake_minimum_required(VERSION 3.5.0)
project(3D-Graphics VERSION 0.1.0 LANGUAGES C CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Set output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}/src/cpp")

# List all source files
set(SOURCES
    ./cpp/main2.cpp
    ./cpp/camera.cpp
    ./cpp/game.cpp
    ./cpp/light.cpp
    ./cpp/object3D.cpp
    ./cpp/pixelArray.cpp
    ./cpp/threads.cpp
    ./cpp/vec3.cpp
    ./cpp/zBuffer.cpp
)

# Compiler and linker options
set(EMSCRIPTEN_OPTIONS
    -sALLOW_MEMORY_GROWTH
    -sUSE_PTHREADS
    -sPTHREAD_POOL_SIZE=10
    -sMODULARIZE
    -sSINGLE_FILE=1
    -sEXPORT_ES6=1
    -sENVIRONMENT=web,worker
    -sEXPORT_NAME="WasmModule"
    -pthread
    -O1
    -flto
    -fapprox-func
    -fno-math-errno
    -fassociative-math
    -freciprocal-math
    -fno-signed-zeros
    -fno-trapping-math
    -fno-rounding-math
    -ffp-contract=fast
)

# Profiling options
# --profiling
# --profiling-funcs
# -fno-inline
# --tracing
# -g

# Define the executable
add_executable(3D-Graphics ${SOURCES})

# Apply compile and link options
target_compile_options(3D-Graphics PRIVATE ${EMSCRIPTEN_OPTIONS})
target_link_options(3D-Graphics PRIVATE ${EMSCRIPTEN_OPTIONS})
