#ifndef FLY_RULER_MODEL_FFI_H
#define FLY_RULER_MODEL_FFI_H

#include <stdarg.h>

// hook of model init
int frmodel_install_hook(int arg_len, ...);

// hook of model uninstall
int frmodel_uninstall_hook(int arg_len, ...);

// get model state
int frmodel_get_state(double *xu, double *xdot);

#endif // FLY_RULER_MODEL_FFI_H