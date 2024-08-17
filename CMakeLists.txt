cmake_minimum_required(VERSION 3.5.0)
project(3D-Graphics-Engine VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_executable(3D-Graphics-Engine main.cpp graphics.cpp)

# set(CMAKE_TOOLCHAIN_FILE /Users/elliottfaa/vcpkg/scripts/buildsystems/vcpkg.cmake CACHE STRING "Vcpkg toolchain file")

if (CMAKE_SYSTEM_NAME STREQUAL Emscripten)
    set(CMAKE_EXECUTABLE_SUFFIX .js)
endif()

target_compile_options(3D-Graphics-Engine PRIVATE -sALLOW_MEMORY_GROWTH -sUSE_PTHREADS)
target_link_options(3D-Graphics-Engine PRIVATE -sALLOW_MEMORY_GROWTH -sUSE_PTHREADS)
