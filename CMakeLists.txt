cmake_minimum_required(VERSION 3.10)
project(cpu_6502)

set(CMAKE_CXX_STANDARD 17)

# Update this to your actual Scoop SDL2 path
set(SDL2_PATH "C:/Users/teore/scoop/apps/sdl2/current")

# Include directories
include_directories(${SDL2_PATH}/include)

# Add compiler definition to handle SDL main
add_definitions(-DSDL_MAIN_HANDLED)

# Add your executable
add_executable(cpu_6502 
    main.cpp
    cpu.cpp
    bus.cpp
    ppu.cpp
    rom_loader.cpp
)

# Link with SDL2 libraries
target_link_libraries(cpu_6502
    ${SDL2_PATH}/lib/SDL2.lib
)

# Copy SDL2.dll to output directory
add_custom_command(TARGET cpu_6502 POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
    "${SDL2_PATH}/lib/SDL2.dll"
    $<TARGET_FILE_DIR:cpu_6502>
)