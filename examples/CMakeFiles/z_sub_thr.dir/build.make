# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.22

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

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/zenoh/actions-runner/_work/devops/devops

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/zenoh/actions-runner/_work/devops/devops

# Include any dependencies generated for this target.
include examples/CMakeFiles/z_sub_thr.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include examples/CMakeFiles/z_sub_thr.dir/compiler_depend.make

# Include the progress variables for this target.
include examples/CMakeFiles/z_sub_thr.dir/progress.make

# Include the compile flags for this target's objects.
include examples/CMakeFiles/z_sub_thr.dir/flags.make

examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o: examples/CMakeFiles/z_sub_thr.dir/flags.make
examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o: examples/z_sub_thr.c
examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o: examples/CMakeFiles/z_sub_thr.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/zenoh/actions-runner/_work/devops/devops/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o"
	cd /home/zenoh/actions-runner/_work/devops/devops/examples && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o -MF CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o.d -o CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o -c /home/zenoh/actions-runner/_work/devops/devops/examples/z_sub_thr.c

examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/z_sub_thr.dir/z_sub_thr.c.i"
	cd /home/zenoh/actions-runner/_work/devops/devops/examples && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/zenoh/actions-runner/_work/devops/devops/examples/z_sub_thr.c > CMakeFiles/z_sub_thr.dir/z_sub_thr.c.i

examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/z_sub_thr.dir/z_sub_thr.c.s"
	cd /home/zenoh/actions-runner/_work/devops/devops/examples && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/zenoh/actions-runner/_work/devops/devops/examples/z_sub_thr.c -o CMakeFiles/z_sub_thr.dir/z_sub_thr.c.s

# Object files for target z_sub_thr
z_sub_thr_OBJECTS = \
"CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o"

# External object files for target z_sub_thr
z_sub_thr_EXTERNAL_OBJECTS =

target/release/examples/z_sub_thr: examples/CMakeFiles/z_sub_thr.dir/z_sub_thr.c.o
target/release/examples/z_sub_thr: examples/CMakeFiles/z_sub_thr.dir/build.make
target/release/examples/z_sub_thr: target/release/libzenohc.so
target/release/examples/z_sub_thr: examples/CMakeFiles/z_sub_thr.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/zenoh/actions-runner/_work/devops/devops/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable ../target/release/examples/z_sub_thr"
	cd /home/zenoh/actions-runner/_work/devops/devops/examples && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/z_sub_thr.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
examples/CMakeFiles/z_sub_thr.dir/build: target/release/examples/z_sub_thr
.PHONY : examples/CMakeFiles/z_sub_thr.dir/build

examples/CMakeFiles/z_sub_thr.dir/clean:
	cd /home/zenoh/actions-runner/_work/devops/devops/examples && $(CMAKE_COMMAND) -P CMakeFiles/z_sub_thr.dir/cmake_clean.cmake
.PHONY : examples/CMakeFiles/z_sub_thr.dir/clean

examples/CMakeFiles/z_sub_thr.dir/depend:
	cd /home/zenoh/actions-runner/_work/devops/devops && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/zenoh/actions-runner/_work/devops/devops /home/zenoh/actions-runner/_work/devops/devops/examples /home/zenoh/actions-runner/_work/devops/devops /home/zenoh/actions-runner/_work/devops/devops/examples /home/zenoh/actions-runner/_work/devops/devops/examples/CMakeFiles/z_sub_thr.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : examples/CMakeFiles/z_sub_thr.dir/depend

