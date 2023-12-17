#ifndef FR_SYSTEM_H
#define FR_SYSTEM_H

#include <stdarg.h>

/// @brief Hook: when system initialization is complete
/// @param arg_len the length of input args
/// @param ... args
/// @return <0 represent occur some error
int frsys_init_hook(int arg_len, ...);

/// @brief Hook: when system is stopped
/// @param arg_len the length of input args
/// @param ... args
/// @return <0 represent occur some error
int frsys_stop_hook(int arg_len, ...);

/// @brief Hook: when system moved forward one frame
/// @param arg_len the length of input args
/// @param ... args
/// @return <0 represent occur some error
int frsys_step_hook(int arg_len, ...);

#endif // FR_SYSTEM_H