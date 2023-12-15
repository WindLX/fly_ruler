#ifndef FLY_RULER_UTILS_FFI_H
#define FLY_RULER_UTILS_FFI_H

typedef enum
{
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL
} InfoLevel;

typedef void (*Logger)(const char *msg, InfoLevel level);

extern Logger frsys_log;

// register logger
void frutils_register_logger(Logger lg);

#endif // FLY_RULER_UTILS_FFI_H