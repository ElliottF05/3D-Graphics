cmake_minimum_required(VERSION 3.0.0)
project(3D-Graphics-Engine)

include(CTest)
enable_testing()

find_package(SFML COMPONENTS system window graphics REQUIRED)

set(SOURCES
    main.cpp
    2d.cpp
    3d.cpp
    window.cpp
)
add_executable(3D-Graphics-Engine ${SOURCES})

# set(CMAKE_TOOLCHAIN_FILE /Users/elliottfaa/vcpkg/scripts/buildsystems/vcpkg.cmake CACHE STRING "Vcpkg toolchain file")

set(CPACK_PROJECT_NAME $3D-Graphics-Engine)
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

target_link_libraries(3D-Graphics-Engine sfml-graphics)