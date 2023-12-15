#ifndef FLY_RULER_UTILS_FFI_H
#define FLY_RULER_UTILS_FFI_H

/// @brief the level of the message
typedef enum
{
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL
} LogLevel;

/// @brief the type of logger callback function
typedef void (*Logger)(const char *msg, LogLevel level);

/// @brief the instance of logger callback function
extern Logger frutils_log;

/// @brief register logger callback function,
///        system will call this function when system initialization is complete
/// @param lg the logger instance transfer from system
void frutils_register_logger(Logger lg);

#endif // FLY_RULER_UTILS_FFI_H