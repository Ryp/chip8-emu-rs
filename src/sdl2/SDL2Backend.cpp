////////////////////////////////////////////////////////////////////////////////
/// chip8-emu
///
/// Copyright (c) 2018 Thibault Schueller
/// This file is distributed under the MIT License
////////////////////////////////////////////////////////////////////////////////

#include "SDL2Backend.h"

#include "chip8/Config.h"
#include "chip8/Cpu.h"
#include "chip8/Display.h"
#include "chip8/Keyboard.h"
#include "chip8/Execution.h"

#include "core/Assert.h"

#include <SDL2/SDL.h>

#include <vector>
#include <iostream>

namespace
{
    void fill_image_buffer(u8* imageOutput, const chip8::CPUState& state, const chip8::Palette& palette, unsigned int scale)
    {
        static constexpr u32 pixelFormatBGRASizeInBytes = 4;

        const u8 primaryColorBGRA[4] = {
            static_cast<u8>(palette.primary.b * 255.f),
            static_cast<u8>(palette.primary.g * 255.f),
            static_cast<u8>(palette.primary.r * 255.f),
            255
        };
        const u8 secondaryColorBGRA[4] = {
            static_cast<u8>(palette.secondary.b * 255.f),
            static_cast<u8>(palette.secondary.g * 255.f),
            static_cast<u8>(palette.secondary.r * 255.f),
            255
        };

        for (unsigned int j = 0; j < chip8::ScreenHeight * scale; j++)
        {
            for (unsigned int i = 0; i < chip8::ScreenWidth * scale; i++)
            {
                const unsigned int pixelIndexFlatDst = j * chip8::ScreenWidth * scale + i;
                const unsigned int pixelOutputOffsetInBytes = pixelIndexFlatDst * pixelFormatBGRASizeInBytes;
                const u8 pixelValue = read_screen_pixel(state, i / scale, j / scale);

                if (pixelValue)
                {
                    imageOutput[pixelOutputOffsetInBytes + 0] = primaryColorBGRA[0];
                    imageOutput[pixelOutputOffsetInBytes + 1] = primaryColorBGRA[1];
                    imageOutput[pixelOutputOffsetInBytes + 2] = primaryColorBGRA[2];
                    imageOutput[pixelOutputOffsetInBytes + 3] = primaryColorBGRA[3];
                }
                else
                {
                    imageOutput[pixelOutputOffsetInBytes + 0] = secondaryColorBGRA[0];
                    imageOutput[pixelOutputOffsetInBytes + 1] = secondaryColorBGRA[1];
                    imageOutput[pixelOutputOffsetInBytes + 2] = secondaryColorBGRA[2];
                    imageOutput[pixelOutputOffsetInBytes + 3] = secondaryColorBGRA[3];
                }
            }
        }
    }
}

namespace sdl2
{
    int execute_main_loop(chip8::CPUState& state, const chip8::EmuConfig& config)
    {
        static constexpr u32 pixelFormatBGRASizeInBytes = 4;
        const unsigned int scale = config.screenScale;
        const unsigned int width = chip8::ScreenWidth * scale;
        const unsigned int height = chip8::ScreenHeight * scale;
        const unsigned int stride = width * pixelFormatBGRASizeInBytes; // No extra space between lines
        const unsigned int size = stride * chip8::ScreenHeight * scale;

        std::vector<unsigned char> image(size);

        Assert(SDL_Init(SDL_INIT_EVERYTHING) == 0, SDL_GetError());

        SDL_Window* win = SDL_CreateWindow("CHIP-8 Emulator", 100, 100, width, height, SDL_WINDOW_SHOWN);
        Assert(win != nullptr, SDL_GetError());

        SDL_Renderer* ren = SDL_CreateRenderer(win, -1, SDL_RENDERER_ACCELERATED);
        Assert(ren != nullptr, SDL_GetError());

        unsigned int rmask, gmask, bmask, amask;
#if SDL_BYTEORDER == SDL_BIG_ENDIAN
        rmask = 0xff000000;
        gmask = 0x00ff0000;
        bmask = 0x0000ff00;
        amask = 0x000000ff;
#else // little endian, like x86
        rmask = 0x000000ff;
        gmask = 0x0000ff00;
        bmask = 0x00ff0000;
        amask = 0xff000000;
#endif

        int depth, pitch;
        depth = 32;
        pitch = stride;

        SDL_Surface* surf = SDL_CreateRGBSurfaceFrom(static_cast<void*>(image.data()), width, height, depth, pitch, rmask, gmask, bmask, amask);
        Assert(surf != nullptr, SDL_GetError());

        unsigned int previousTimeMs = SDL_GetTicks();
        bool shouldExit = false;

        while (!shouldExit)
        {
            // Poll events
            SDL_Event sdlEvent;
            while (SDL_PollEvent(&sdlEvent))
            {
                switch (sdlEvent.type)
                {
                    case SDL_QUIT:
                        shouldExit = true;
                        break;
                    case SDL_KEYDOWN:
                        if (sdlEvent.key.keysym.sym == SDLK_ESCAPE)
                            shouldExit = true;
                        break;
                }
            }

            // Get keyboard state
            const unsigned char* sdlKeyStates = SDL_GetKeyboardState(nullptr);
            chip8::set_key_pressed(state, 0x1, sdlKeyStates[SDL_SCANCODE_1]);
            chip8::set_key_pressed(state, 0x2, sdlKeyStates[SDL_SCANCODE_2]);
            chip8::set_key_pressed(state, 0x3, sdlKeyStates[SDL_SCANCODE_3]);
            chip8::set_key_pressed(state, 0xC, sdlKeyStates[SDL_SCANCODE_4]);
            chip8::set_key_pressed(state, 0x4, sdlKeyStates[SDL_SCANCODE_Q]);
            chip8::set_key_pressed(state, 0x5, sdlKeyStates[SDL_SCANCODE_W]);
            chip8::set_key_pressed(state, 0x6, sdlKeyStates[SDL_SCANCODE_E]);
            chip8::set_key_pressed(state, 0xD, sdlKeyStates[SDL_SCANCODE_R]);
            chip8::set_key_pressed(state, 0x7, sdlKeyStates[SDL_SCANCODE_A]);
            chip8::set_key_pressed(state, 0x8, sdlKeyStates[SDL_SCANCODE_S]);
            chip8::set_key_pressed(state, 0x9, sdlKeyStates[SDL_SCANCODE_D]);
            chip8::set_key_pressed(state, 0xE, sdlKeyStates[SDL_SCANCODE_F]);
            chip8::set_key_pressed(state, 0xA, sdlKeyStates[SDL_SCANCODE_Z]);
            chip8::set_key_pressed(state, 0x0, sdlKeyStates[SDL_SCANCODE_X]);
            chip8::set_key_pressed(state, 0xB, sdlKeyStates[SDL_SCANCODE_C]);
            chip8::set_key_pressed(state, 0xF, sdlKeyStates[SDL_SCANCODE_V]);

            unsigned int currentTimeMs = SDL_GetTicks();
            unsigned int deltaTimeMs = currentTimeMs - previousTimeMs;

            chip8::execute_step(config, state, deltaTimeMs);

            fill_image_buffer(image.data(), state, config.palette, scale);

            // Draw
            SDL_Texture* tex = SDL_CreateTextureFromSurface(ren, surf);
            Assert(tex != nullptr, SDL_GetError());

            SDL_RenderClear(ren);
            SDL_RenderCopy(ren, tex, nullptr, nullptr);

            // Present
            SDL_RenderPresent(ren);

            SDL_DestroyTexture(tex);

            previousTimeMs = currentTimeMs;
        }

        SDL_FreeSurface(surf);
        SDL_DestroyRenderer(ren);
        SDL_DestroyWindow(win);
        SDL_Quit();

        return EXIT_SUCCESS;
    }
}
