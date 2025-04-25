#include <windows.h>
#include <SDL.h>
#include <SDL_syswm.h>
#include <iostream>

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE, LPSTR, int nCmdShow) {
    // Register window class
    const char CLASS_NAME[] = "SDLWinAPIWindow";

    WNDCLASS wc = {};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = CLASS_NAME;

    RegisterClass(&wc);

    // Create menu
    HMENU hMenu = CreateMenu();
    HMENU hFileMenu = CreatePopupMenu();
    AppendMenu(hFileMenu, MF_STRING, 1, "Open");
    AppendMenu(hFileMenu, MF_STRING, 2, "Exit");
    AppendMenu(hMenu, MF_POPUP, (UINT_PTR)hFileMenu, "File");

    HMENU hEditMenu = CreatePopupMenu();
    AppendMenu(hEditMenu, MF_STRING, 3, "Undo");
    AppendMenu(hMenu, MF_POPUP, (UINT_PTR)hEditMenu, "Edit");

    // Create window
    HWND hwnd = CreateWindowEx(
        0, CLASS_NAME, "SDL + WinAPI Menu",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, 800, 600,
        NULL, hMenu, hInstance, NULL
    );

    ShowWindow(hwnd, nCmdShow);

    // Initialize SDL
    SDL_Init(SDL_INIT_VIDEO);

    SDL_Window* sdlWindow = SDL_CreateWindowFrom((void*)hwnd);
    SDL_Renderer* renderer = SDL_CreateRenderer(sdlWindow, -1, SDL_RENDERER_ACCELERATED);

    // Main loop
    bool running = true;
    MSG msg = {};
    while (running) {
        while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE)) {
            if (msg.message == WM_QUIT) {
                running = false;
            }
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }

        // SDL rendering
        SDL_SetRenderDrawColor(renderer, 30, 30, 100, 255);
        SDL_RenderClear(renderer);

        // Draw something (blue background)
        SDL_RenderPresent(renderer);
    }

    // Cleanup
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(sdlWindow);
    SDL_Quit();

    return 0;
}

// WinAPI Window Procedure
LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
    case WM_COMMAND:
        switch (LOWORD(wParam)) {
        case 1: MessageBox(hwnd, "Open selected", "File Menu", MB_OK); break;
        case 2: PostQuitMessage(0); break;
        case 3: MessageBox(hwnd, "Undo selected", "Edit Menu", MB_OK); break;
        }
        return 0;

    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    }

    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}
