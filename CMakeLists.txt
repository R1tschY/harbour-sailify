cmake_minimum_required(VERSION 3.9)

project(sailify VERSION 0.1.0 LANGUAGES C CXX)
set(PACKAGE_NAME harbour-sailify)

set(CMAKE_CXX_STANDARD 17)
set(SAILFISHOS ON)
set(BUILD_SHARED_LIBS OFF)

include(FindPkgConfig)


# Sailfish App
pkg_search_module(SAILFISHAPP sailfishapp REQUIRED)

# PulseAudio
pkg_search_module(PULSE libpulse REQUIRED)
pkg_search_module(PULSE_SIMPLE libpulse-simple REQUIRED)

## Set RPATH
set(CMAKE_BUILD_WITH_INSTALL_RPATH OFF)
set(CMAKE_INSTALL_RPATH_USE_LINK_PATH OFF)
set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/share/${PACKAGE_NAME}/lib") # "${LIB_INSTALL_DIR}")

# booster support
set(CMAKE_ENABLE_EXPORTS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_VISIBILITY_INLINES_HIDDEN hidden)

# Qt
find_package(Qt5 REQUIRED COMPONENTS Core DBus Network Gui Quick)
add_definitions(-DQT_DEPRECATED_WARNINGS -DQT_DISABLE_DEPRECATED_BEFORE=0x050600 -DQT_USE_QSTRINGBUILDER)
if(NOT ${CMAKE_BUILD_TYPE} STREQUAL "Debug")
  add_definitions(-DQT_NO_DEBUG_OUTPUT -DQT_NO_DEBUG)
endif()
add_definitions(-DSAILFISHOS)

set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# cbindgen
set(sailifyplayer_HEADER "${CMAKE_CURRENT_BINARY_DIR}/sailifyplayer.h")
file(GLOB sailifyplayer_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.rs")
add_custom_target(
  sailifyplayer_header
  COMMENT cbindgen sailifyplayer
  COMMAND cbindgen --crate sailifyplayer --output "${sailifyplayer_HEADER}"
  BYPRODUCTS ${sailifyplayer_HEADER}
  DEPENDS ${sailifyplayer_SOURCES}
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)
set_property(SOURCE "${sailifyplayer_HEADER}" PROPERTY SKIP_AUTOMOC ON)

# sailifyplayer
set(CARGO_ARGS)
# TODO: other profiles exist
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(TARGET_DIR "debug")
else()
  list(APPEND CARGO_ARGS --release)
  set(TARGET_DIR "release")
endif()

add_library(sailifyplayer STATIC IMPORTED)
set_property(TARGET sailifyplayer PROPERTY IMPORTED_LOCATION "${CMAKE_CURRENT_BINARY_DIR}/libsailifyplayer.a")
target_link_libraries(sailifyplayer INTERFACE
    -lpthread -ldl -lm
    ${PULSE_LDFLAGS} ${PULSE_SIMPLE_LDFLAGS})

#add_custom_target(
#  sailifyplayer
#  COMMENT Cargo sailifyplayer
#  COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/sailifyplayer cargo build ${CARGO_ARGS}
#  BYPRODUCTS ${SAILIFYPLAYER_LIB}
#  DEPENDS ${sailifyplayer_SOURCES}
#  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
# )

# include(ExternalProject)
# set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/sailifyplayer2)
# ExternalProject_Add(
#     sailifyplayer2
#     DOWNLOAD_COMMAND ""
#     CONFIGURE_COMMAND ""
#     BUILD_COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build ${CARGO_ARGS}
#     BINARY_DIR "${CMAKE_SOURCE_DIR}/common-rust"
#     INSTALL_COMMAND ""
#     LOG_BUILD ON)


# App
set(CONFIG_SOURCE src/sailify-config.cpp)
file(GLOB_RECURSE sailify_QML_SOURCES "qml/*.qml")

set(SAILIFY_SOURCES
    src/sailify.cpp

    ${sailify_QML_SOURCES}
    ${sailifyplayer_HEADER}
    ${CMAKE_CURRENT_BINARY_DIR}/${CONFIG_SOURCE}
)

configure_file(
    ${CONFIG_SOURCE}.in ${CONFIG_SOURCE}
    @ONLY
)

add_executable(sailify ${SAILIFY_SOURCES})
set_target_properties(sailify
  PROPERTIES
  OUTPUT_NAME ${PACKAGE_NAME}
)
add_dependencies(sailify sailifyplayer_header)
target_include_directories(sailify
    PRIVATE
        ${SAILFISHAPP_INCLUDE_DIRS}
)
target_link_libraries(sailify
  Qt5::Core Qt5::DBus Qt5::Network Qt5::Gui Qt5::Quick
  ${SAILFISHAPP_LDFLAGS}

  sailifyplayer
)

install(TARGETS sailify
    RUNTIME DESTINATION bin
)
install(DIRECTORY qml
    DESTINATION share/${PACKAGE_NAME}
)
# install(DIRECTORY translations
#     DESTINATION share/${PACKAGE_NAME}
#     FILES_MATCHING PATTERN "*.qm"
# )
install(FILES ${PACKAGE_NAME}.desktop
    DESTINATION share/applications
)

foreach(size 86 108 128 172)
    install(FILES res/${size}x${size}/${PACKAGE_NAME}.png
        DESTINATION share/icons/hicolor/${size}x${size}/apps
    )
endforeach()

file(WRITE "${CMAKE_BINARY_DIR}/QtCreatorDeployment.txt"
    "${CMAKE_INSTALL_PREFIX}\n${CMAKE_BINARY_DIR}/sailify:bin\n")

