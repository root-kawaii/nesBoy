cmake_minimum_required(VERSION 3.10)
project(cpu_6502)

# Use C++11
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add executable
add_executable(cpu_6502 main.cpp)
