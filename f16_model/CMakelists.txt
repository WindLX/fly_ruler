cmake_minimum_required(VERSION 3.15)

project(f16_model)

# 设置C++标准
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

# 设置头文件目录
include_directories(include)

# 查找并添加源文件
file(GLOB SOURCES "src/*.c")

# 添加可执行文件
add_executable(f16_model ${SOURCES})
