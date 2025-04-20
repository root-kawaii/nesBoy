cmake_minimum_required(VERSION 3.10)
project(cpu_6502)

# Use C++11
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Specify the header include directories
include_directories(${PROJECT_SOURCE_DIR})  # Assumes headers are in the project root directory.

# Add executable
add_executable(cpu_6502 
    main.cpp      # Your main program file
    cpu.cpp   # CPU6502 implementation file
    bus.cpp       # Bus implementation file
)
