cmake_minimum_required(VERSION 3.19)
if(COMMAND cmake_policy)
    cmake_policy(SET CMP0003 NEW)
endif(COMMAND cmake_policy)

set(CMAKE_EXPORT_COMPILE_COMMANDS YES)

project(lmmp)

find_package(OpenGL)

set(IMGUI_INCLUDE_DIR ${CMAKE_SOURCE_DIR}/extern/imgui)
file(GLOB IMGUI_SOURCES ${IMGUI_INCLUDE_DIR}/*.cpp)
# list(FILTER IMGUI_SOURCES EXCLUDE REGEX ".*_demo.cpp")

add_compile_definitions(IMGUI_IMPL_OPENGL_LOADER_CUSTOM=<epoxy/gl.h>)

add_library(imgui STATIC ${IMGUI_SOURCES} ${IMGUI_INCLUDE_DIR}/backends/imgui_impl_sdl.cpp ${IMGUI_INCLUDE_DIR}/backends/imgui_impl_opengl3.cpp)
target_include_directories(imgui PRIVATE /usr/include/SDL2)
target_link_libraries(imgui PRIVATE SDL2 OpenGL epoxy)
include_directories(${IMGUI_INCLUDE_DIR} ${IMGUI_INCLUDE_DIR}/backends)

add_executable(lmmp
    src/main.cpp
)
target_link_libraries(lmmp PRIVATE imgui SDL2 OpenGL epoxy)
