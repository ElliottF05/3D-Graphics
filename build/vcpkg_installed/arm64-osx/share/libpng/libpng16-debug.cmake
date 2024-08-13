#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "png_static" for configuration "Debug"
set_property(TARGET png_static APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(png_static PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "ASM;C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/debug/lib/libpng16d.a"
  )

list(APPEND _cmake_import_check_targets png_static )
list(APPEND _cmake_import_check_files_for_png_static "${_IMPORT_PREFIX}/debug/lib/libpng16d.a" )

# Import target "png_framework" for configuration "Debug"
set_property(TARGET png_framework APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(png_framework PROPERTIES
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/debug/lib/png.framework/Versions/1.6.43/png"
  IMPORTED_SONAME_DEBUG "@rpath/png.framework/Versions/1.6.43/png"
  )

list(APPEND _cmake_import_check_targets png_framework )
list(APPEND _cmake_import_check_files_for_png_framework "${_IMPORT_PREFIX}/debug/lib/png.framework/Versions/1.6.43/png" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
