// main.cpp
#include <SDL2/SDL_main.h>
#include <SDL2/SDL.h>
#include "bus.h"
#include "cpu.h"
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
    bus.cpu->reset();

    bool running = true;
    SDL_Event ev;
    int colorIndex = 0;
    int startFrame = 0;
    int timePassed = 0;
    int frameTime;

    while (running) {
        // --- Emulate one frame ---
        // while (!bus.ppu->isFrameComplete()) {
        //     bus.cpu->step();
        // }
        bus.ppu->resetFrameComplete();


        // --- Convert PPU framebuffer indices to actual pixels ---
        void* pixels;
        int pitch;
        SDL_LockTexture(tex, nullptr, &pixels, &pitch);
        auto frame = bus.ppu->getFrame();  // returns array<uint8_t,256*240>
        Uint32* dst = (Uint32*)pixels;

        for (int i = 0; i < NES_WIDTH * NES_HEIGHT; ++i) {
            // uint8_t idx = frame[i];         // 0–3 in our minimal demo
            dst[i] = nesPalette[colorIndex];   // ARGB color
        }

        frameTime = SDL_GetTicks() - startFrame;
        std::cout << timePassed << std::endl;
        if(timePassed > 10000000 ){
            colorIndex += 1;
            timePassed = 0;
        }
        if (FRAME_DELAY > frameTime) {
            SDL_Delay(FRAME_DELAY - frameTime);
        }

        
        SDL_UnlockTexture(tex);

        // --- Render to screen ---
        SDL_RenderClear(ren);
        SDL_RenderCopy(ren, tex, nullptr, nullptr);
        SDL_RenderPresent(ren);

        running = true;
        // --- Poll window events ---
        while (SDL_PollEvent(&ev)) {
            if (ev.type == SDL_QUIT)
                running = false;
        }
        timePassed += frameTime;
    }

    SDL_DestroyTexture(tex);
    SDL_DestroyRenderer(ren);
    SDL_DestroyWindow(win);
    SDL_Quit();
    return 0;
}
