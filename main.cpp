// main.cpp
#include <SDL2/SDL_main.h>
#include <SDL2/SDL.h>
#include "bus.h"
#include "cpu.h"
#include "rom_loader.h"
#include <iostream>

#ifdef main
#undef main
#endif

static const int NES_WIDTH  = 256;
static const int NES_HEIGHT = 240;
static const int SCALE      = 3;    // scale up to 768×720
const int FPS = 60;
const int FRAME_DELAY = 1000 / FPS;

// A simple NES palette (RGB triples) for indices 0–3 of each 8×8 tile.
// In reality you’ll use 64 entries per palette + attribute logic; this is just demo.
Uint32 nesPalette[4] = {
    0xFF7C7C7C, // gray
    0xFF0000FF, // red
    0xFF00FF00, // green
    0xFFFF0000  // blue
};



int main(int argc, char** argv) {
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window*   win  = SDL_CreateWindow("NES Emulator",
                           SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
                           NES_WIDTH*SCALE, NES_HEIGHT*SCALE,
                           0);
    SDL_Renderer* ren  = SDL_CreateRenderer(win, -1, SDL_RENDERER_ACCELERATED);
    SDL_Texture*  tex  = SDL_CreateTexture(ren,
                           SDL_PIXELFORMAT_ARGB8888,
                           SDL_TEXTUREACCESS_STREAMING,
                           NES_WIDTH, NES_HEIGHT);

    Bus bus;                // wires up CPU/PPU
    RomLoader romLoader;
    bus.cpu->reset();

    bool running = true;
    SDL_Event ev;
    int colorIndex = 0;
    Uint32 colorTimer = 0;
    Uint32 frameStart;
    int frameTime;

    romLoader.loadRom("ff.nes");

    while (running) {
        frameStart = SDL_GetTicks();
        
        // --- Emulate one frame ---
        // Commented out for now until CPU/PPU are ready
        // while (!bus.ppu->isFrameComplete()) {
        //     bus.cpu->step();
        // }
        // bus.ppu->resetFrameComplete();

        // --- Convert PPU framebuffer indices to actual pixels ---
    
        void* pixels;
        int pitch;
        SDL_LockTexture(tex, nullptr, &pixels, &pitch);
        Uint32* dst = (Uint32*)pixels;

        // Fill screen with current palette color
        for (int i = 0; i < NES_WIDTH * NES_HEIGHT; ++i) {
            dst[i] = nesPalette[colorIndex % 4];
        }
        SDL_UnlockTexture(tex);

        // --- Render to screen ---
        SDL_RenderClear(ren);
        SDL_RenderCopy(ren, tex, nullptr, nullptr);
        SDL_RenderPresent(ren);

        // --- Poll window events ---
        while (SDL_PollEvent(&ev)) {
            if (ev.type == SDL_QUIT)
                running = false;
        }
        
        // --- Frame timing control ---
        frameTime = SDL_GetTicks() - frameStart;
        if (FRAME_DELAY > frameTime) {
            SDL_Delay(FRAME_DELAY - frameTime);
        }
        
        // Update color every 2 seconds
        colorTimer += frameTime;
        if (colorTimer >= 2000) {
            colorIndex++;
            colorTimer = 0;
            std::cout << "Changed color to index: " << colorIndex % 4 << std::endl;
        }
    }

    SDL_DestroyTexture(tex);
    SDL_DestroyRenderer(ren);
    SDL_DestroyWindow(win);
    SDL_Quit();
    return 0;
}