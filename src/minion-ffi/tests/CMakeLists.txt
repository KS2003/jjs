cmake_minimum_required(VERSION 3.12)
project(JjsMinionFfiTests)

set(MINION_HEADER ${CMAKE_CURRENT_BINARY_DIR}/minion-ffi.h)
set(CRATE ${CMAKE_SOURCE_DIR}/..)
set(JJS_SRC_ROOT ${CMAKE_SOURCE_DIR}/../../..)
set(JJS_TARGET_DIR ${JJS_SRC_ROOT}/target)
add_custom_target(minion_ffi_headers
        COMMAND cp ${JJS_TARGET_DIR}/minion-ffi.h ${CMAKE_CURRENT_BINARY_DIR}/minion-ffi.h &&
        cp ${JJS_TARGET_DIR}/minion-ffi-prepend.h ${CMAKE_CURRENT_BINARY_DIR}/minion-ffi-prepend.h)
add_custom_target(minion_ffi
        COMMAND cargo build WORKING_DIRECTORY ${CRATE})
add_dependencies(minion_ffi_headers minion_ffi)
add_executable(mft tests.c runner.c)
target_include_directories(mft PUBLIC ${CMAKE_CURRENT_BINARY_DIR})
target_link_libraries(mft PUBLIC ${JJS_TARGET_DIR}/debug/libminion_ffi.a pthread rt dl m)
add_dependencies(mft minion_ffi minion_ffi_headers)
