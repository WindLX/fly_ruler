#ifndef FR_UTIL
#define FR_UTIL

/// @brief the level of the message
typedef enum
{
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
} LogLevel;

/// @brief the type of logger callback function
typedef void (*Logger)(const char *msg, LogLevel level);

/// @brief the instance of logger callback function
extern Logger frutil_log;

/// @brief register logger callback function,
///        system will call this function before loading plugins or installing models
/// @param lg the logger instance transfer from system
void frutil_register_logger(Logger lg);

#endif // FR_UTIL