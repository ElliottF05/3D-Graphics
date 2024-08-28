cmake_minimum_required(VERSION 3.5.0)
project(3D-Graphics VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_executable(3D-Graphics main.cpp graphics.cpp threads.cpp)

# set(CMAKE_TOOLCHAIN_FILE /Users/elliottfaa/vcpkg/scripts/buildsystems/vcpkg.cmake CACHE STRING "Vcpkg toolchain file")

if (CMAKE_SYSTEM_NAME STREQUAL Emscripten)
    set(CMAKE_EXECUTABLE_SUFFIX .js)
endif()

target_compile_options(3D-Graphics PRIVATE -sALLOW_MEMORY_GROWTH -sUSE_PTHREADS -sPTHREAD_POOL_SIZE=30 -pthread -O3 -flto -fapprox-func -fno-math-errno -fassociative-math -freciprocal-math -fno-signed-zeros -fno-trapping-math -fno-rounding-math -ffp-contract=fast)
target_link_options(3D-Graphics PRIVATE -sALLOW_MEMORY_GROWTH -sUSE_PTHREADS -sPTHREAD_POOL_SIZE=30 -pthread -O3 -flto  -fapprox-func -fno-math-errno -fassociative-math -freciprocal-math -fno-signed-zeros -fno-trapping-math -fno-rounding-math -ffp-contract=fast)
