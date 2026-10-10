cmake_minimum_required(VERSION 3.21)

project(iconlookup LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Gui)
find_package(KF6IconThemes REQUIRED)

add_executable(iconlookup iconlookup.cpp)

target_link_libraries(iconlookup PRIVATE
    Qt6::Gui
    KF6::IconThemes
)
