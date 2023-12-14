#ifndef FLY_RULER_FFI_H
#define FLY_RULER_FFI_H

#include <stdarg.h>

// 系统初始化的钩子
void systemInitHook(int argLen, ...);

// 系统终止时的钩子
void systemStopHook(int argLen, ...);

// 系统调用一步时的钩子
void systemStepHook(int argLen, ...);

// 获取飞行器状态值
void getState(double *xu, double *xdot);

typedef enum
{
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL
} InfoLevel;

typedef void (*LogCallback)(char *msg, InfoLevel level);
extern LogCallback log = NULL;

// 设置打印消息的回调函数
void setLogCallback(LogCallback cb);

#endif // FLY_RULER_FFI_H