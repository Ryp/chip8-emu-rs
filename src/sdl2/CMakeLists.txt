#///////////////////////////////////////////////////////////////////////////////
#// chip8-emu
#//
#// Copyright (c) 2018 Thibault Schueller
#// This file is distributed under the MIT License
#///////////////////////////////////////////////////////////////////////////////

set(target chip8emu_sdl2)
set(CHIP8EMU_SDL2_BIN ${target} PARENT_SCOPE)

add_library(${target} ${CHIP8EMU_BUILD_TYPE})

target_sources(${target} PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/SDL2Backend.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/SDL2Backend.h
    ${CMAKE_CURRENT_SOURCE_DIR}/SDL2Export.h
)

target_link_libraries(${target} PRIVATE
    ${CHIP8EMU_CORE_BIN}
    ${CHIP8EMU_EMU_BIN}
    SDL2
)

reaper_configure_library(${target} "SDL2")
