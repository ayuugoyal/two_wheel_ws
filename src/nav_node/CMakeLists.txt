cmake_minimum_required(VERSION 3.5)
project(navigation_node)

find_package(ament_cmake REQUIRED)

install(
    DIRECTORY src
    DESTINATION lib/${PROJECT_NAME}
)

ament_package()
