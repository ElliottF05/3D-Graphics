cmake_minimum_required(VERSION 3.5.0)
project(3D-Graphics VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_BINARY_DIR "${PROJECT_SOURCE_DIR}/src/cpp")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}/src/cpp")

add_executable(3D-Graphics ./cpp/main2.cpp ./cpp/camera.cpp ./cpp/game.cpp ./cpp/object3D.cpp ./cpp/pixelArray.cpp ./cpp/threads.cpp ./cpp/vec3.cpp)

target_compile_options(3D-Graphics PRIVATE -sALLOW_MEMORY_GROWTH -sUSE_PTHREADS -sPTHREAD_POOL_SIZE=20 -sMODULARIZE -sSINGLE_FILE=1 -sEXPORT_ES6=1 -sENVIRONMENT=web,worker -sEXPORT_NAME="WasmModule" -pthread -O3 -flto -fapprox-func -fno-math-errno -fassociative-math -freciprocal-math -fno-signed-zeros -fno-trapping-math -fno-rounding-math -ffp-contract=fast)
target_link_options(3D-Graphics PRIVATE -sALLOW_MEMORY_GROWTH -sUSE_PTHREADS -sPTHREAD_POOL_SIZE=20 -sMODULARIZE -sSINGLE_FILE=1 -sEXPORT_ES6=1 -sENVIRONMENT=web,worker -sEXPORT_NAME="WasmModule" -pthread -O3 -flto  -fapprox-func -fno-math-errno -fassociative-math -freciprocal-math -fno-signed-zeros -fno-trapping-math -fno-rounding-math -ffp-contract=fast)
