# CMAKE generated file: DO NOT EDIT!
# Generated by "MinGW Makefiles" Generator, CMake Version 3.31

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

SHELL = cmd.exe

# The CMake executable.
CMAKE_COMMAND = C:\Users\teore\scoop\apps\cmake\3.31.6\bin\cmake.exe

# The command to remove a file.
RM = C:\Users\teore\scoop\apps\cmake\3.31.6\bin\cmake.exe -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = C:\Users\teore\Desktop\cpu_6502

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = C:\Users\teore\Desktop\cpu_6502\build

# Include any dependencies generated for this target.
include CMakeFiles/cpu_6502.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/cpu_6502.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/cpu_6502.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/cpu_6502.dir/flags.make

CMakeFiles/cpu_6502.dir/codegen:
.PHONY : CMakeFiles/cpu_6502.dir/codegen

CMakeFiles/cpu_6502.dir/main.cpp.obj: CMakeFiles/cpu_6502.dir/flags.make
CMakeFiles/cpu_6502.dir/main.cpp.obj: CMakeFiles/cpu_6502.dir/includes_CXX.rsp
CMakeFiles/cpu_6502.dir/main.cpp.obj: C:/Users/teore/Desktop/cpu_6502/main.cpp
CMakeFiles/cpu_6502.dir/main.cpp.obj: CMakeFiles/cpu_6502.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/cpu_6502.dir/main.cpp.obj"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpu_6502.dir/main.cpp.obj -MF CMakeFiles\cpu_6502.dir\main.cpp.obj.d -o CMakeFiles\cpu_6502.dir\main.cpp.obj -c C:\Users\teore\Desktop\cpu_6502\main.cpp

CMakeFiles/cpu_6502.dir/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/cpu_6502.dir/main.cpp.i"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E C:\Users\teore\Desktop\cpu_6502\main.cpp > CMakeFiles\cpu_6502.dir\main.cpp.i

CMakeFiles/cpu_6502.dir/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/cpu_6502.dir/main.cpp.s"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S C:\Users\teore\Desktop\cpu_6502\main.cpp -o CMakeFiles\cpu_6502.dir\main.cpp.s

CMakeFiles/cpu_6502.dir/cpu.cpp.obj: CMakeFiles/cpu_6502.dir/flags.make
CMakeFiles/cpu_6502.dir/cpu.cpp.obj: CMakeFiles/cpu_6502.dir/includes_CXX.rsp
CMakeFiles/cpu_6502.dir/cpu.cpp.obj: C:/Users/teore/Desktop/cpu_6502/cpu.cpp
CMakeFiles/cpu_6502.dir/cpu.cpp.obj: CMakeFiles/cpu_6502.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/cpu_6502.dir/cpu.cpp.obj"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpu_6502.dir/cpu.cpp.obj -MF CMakeFiles\cpu_6502.dir\cpu.cpp.obj.d -o CMakeFiles\cpu_6502.dir\cpu.cpp.obj -c C:\Users\teore\Desktop\cpu_6502\cpu.cpp

CMakeFiles/cpu_6502.dir/cpu.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/cpu_6502.dir/cpu.cpp.i"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E C:\Users\teore\Desktop\cpu_6502\cpu.cpp > CMakeFiles\cpu_6502.dir\cpu.cpp.i

CMakeFiles/cpu_6502.dir/cpu.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/cpu_6502.dir/cpu.cpp.s"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S C:\Users\teore\Desktop\cpu_6502\cpu.cpp -o CMakeFiles\cpu_6502.dir\cpu.cpp.s

CMakeFiles/cpu_6502.dir/bus.cpp.obj: CMakeFiles/cpu_6502.dir/flags.make
CMakeFiles/cpu_6502.dir/bus.cpp.obj: CMakeFiles/cpu_6502.dir/includes_CXX.rsp
CMakeFiles/cpu_6502.dir/bus.cpp.obj: C:/Users/teore/Desktop/cpu_6502/bus.cpp
CMakeFiles/cpu_6502.dir/bus.cpp.obj: CMakeFiles/cpu_6502.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/cpu_6502.dir/bus.cpp.obj"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpu_6502.dir/bus.cpp.obj -MF CMakeFiles\cpu_6502.dir\bus.cpp.obj.d -o CMakeFiles\cpu_6502.dir\bus.cpp.obj -c C:\Users\teore\Desktop\cpu_6502\bus.cpp

CMakeFiles/cpu_6502.dir/bus.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/cpu_6502.dir/bus.cpp.i"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E C:\Users\teore\Desktop\cpu_6502\bus.cpp > CMakeFiles\cpu_6502.dir\bus.cpp.i

CMakeFiles/cpu_6502.dir/bus.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/cpu_6502.dir/bus.cpp.s"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S C:\Users\teore\Desktop\cpu_6502\bus.cpp -o CMakeFiles\cpu_6502.dir\bus.cpp.s

CMakeFiles/cpu_6502.dir/ppu.cpp.obj: CMakeFiles/cpu_6502.dir/flags.make
CMakeFiles/cpu_6502.dir/ppu.cpp.obj: CMakeFiles/cpu_6502.dir/includes_CXX.rsp
CMakeFiles/cpu_6502.dir/ppu.cpp.obj: C:/Users/teore/Desktop/cpu_6502/ppu.cpp
CMakeFiles/cpu_6502.dir/ppu.cpp.obj: CMakeFiles/cpu_6502.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/cpu_6502.dir/ppu.cpp.obj"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpu_6502.dir/ppu.cpp.obj -MF CMakeFiles\cpu_6502.dir\ppu.cpp.obj.d -o CMakeFiles\cpu_6502.dir\ppu.cpp.obj -c C:\Users\teore\Desktop\cpu_6502\ppu.cpp

CMakeFiles/cpu_6502.dir/ppu.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/cpu_6502.dir/ppu.cpp.i"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E C:\Users\teore\Desktop\cpu_6502\ppu.cpp > CMakeFiles\cpu_6502.dir\ppu.cpp.i

CMakeFiles/cpu_6502.dir/ppu.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/cpu_6502.dir/ppu.cpp.s"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S C:\Users\teore\Desktop\cpu_6502\ppu.cpp -o CMakeFiles\cpu_6502.dir\ppu.cpp.s

CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj: CMakeFiles/cpu_6502.dir/flags.make
CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj: CMakeFiles/cpu_6502.dir/includes_CXX.rsp
CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj: C:/Users/teore/Desktop/cpu_6502/rom_loader.cpp
CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj: CMakeFiles/cpu_6502.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building CXX object CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj -MF CMakeFiles\cpu_6502.dir\rom_loader.cpp.obj.d -o CMakeFiles\cpu_6502.dir\rom_loader.cpp.obj -c C:\Users\teore\Desktop\cpu_6502\rom_loader.cpp

CMakeFiles/cpu_6502.dir/rom_loader.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/cpu_6502.dir/rom_loader.cpp.i"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E C:\Users\teore\Desktop\cpu_6502\rom_loader.cpp > CMakeFiles\cpu_6502.dir\rom_loader.cpp.i

CMakeFiles/cpu_6502.dir/rom_loader.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/cpu_6502.dir/rom_loader.cpp.s"
	C:\Users\teore\scoop\apps\mingw\current\bin\g++.exe $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S C:\Users\teore\Desktop\cpu_6502\rom_loader.cpp -o CMakeFiles\cpu_6502.dir\rom_loader.cpp.s

# Object files for target cpu_6502
cpu_6502_OBJECTS = \
"CMakeFiles/cpu_6502.dir/main.cpp.obj" \
"CMakeFiles/cpu_6502.dir/cpu.cpp.obj" \
"CMakeFiles/cpu_6502.dir/bus.cpp.obj" \
"CMakeFiles/cpu_6502.dir/ppu.cpp.obj" \
"CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj"

# External object files for target cpu_6502
cpu_6502_EXTERNAL_OBJECTS =

cpu_6502.exe: CMakeFiles/cpu_6502.dir/main.cpp.obj
cpu_6502.exe: CMakeFiles/cpu_6502.dir/cpu.cpp.obj
cpu_6502.exe: CMakeFiles/cpu_6502.dir/bus.cpp.obj
cpu_6502.exe: CMakeFiles/cpu_6502.dir/ppu.cpp.obj
cpu_6502.exe: CMakeFiles/cpu_6502.dir/rom_loader.cpp.obj
cpu_6502.exe: CMakeFiles/cpu_6502.dir/build.make
cpu_6502.exe: C:/Users/teore/scoop/apps/sdl2/current/lib/SDL2.lib
cpu_6502.exe: CMakeFiles/cpu_6502.dir/linkLibs.rsp
cpu_6502.exe: CMakeFiles/cpu_6502.dir/objects1.rsp
cpu_6502.exe: CMakeFiles/cpu_6502.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Linking CXX executable cpu_6502.exe"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles\cpu_6502.dir\link.txt --verbose=$(VERBOSE)
	C:\Users\teore\scoop\apps\cmake\3.31.6\bin\cmake.exe -E copy_if_different C:/Users/teore/scoop/apps/sdl2/current/lib/SDL2.dll C:/Users/teore/Desktop/cpu_6502/build

# Rule to build all files generated by this target.
CMakeFiles/cpu_6502.dir/build: cpu_6502.exe
.PHONY : CMakeFiles/cpu_6502.dir/build

CMakeFiles/cpu_6502.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles\cpu_6502.dir\cmake_clean.cmake
.PHONY : CMakeFiles/cpu_6502.dir/clean

CMakeFiles/cpu_6502.dir/depend:
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" C:\Users\teore\Desktop\cpu_6502 C:\Users\teore\Desktop\cpu_6502 C:\Users\teore\Desktop\cpu_6502\build C:\Users\teore\Desktop\cpu_6502\build C:\Users\teore\Desktop\cpu_6502\build\CMakeFiles\cpu_6502.dir\DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/cpu_6502.dir/depend

