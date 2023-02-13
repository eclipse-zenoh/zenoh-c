cmake_minimum_required(VERSION 3.16)
project(
	zenohc
	VERSION 0.7.0.1
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)

include(helpers.cmake)
declare_target_projectname_debug() # create target 'zenohc_debug' to show generated expression values with 'debug_printfoo a'

include(library.cmake)
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
	include(rootproject.cmake)
endif()