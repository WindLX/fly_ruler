#ifndef FLY_RULER_SYSTEM_FFI_H
#define FLY_RULER_SYSTEM_FFI_H

#include <stdarg.h>

// hook of system init
int frsys_init_hook(int arg_len, ...);

// hook of system stop
int frsys_stop_hook(int arg_len, ...);

// hook of system step
int frsys_step_hook(int arg_len, ...);

#endif // FLY_RULER_SYSTEM_FFI_H