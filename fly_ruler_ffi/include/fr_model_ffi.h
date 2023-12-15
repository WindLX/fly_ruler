#ifndef FR_MODEL_FFI_H
#define FR_MODEL_FFI_H

#include <stdarg.h>

/// @brief Hook: when model is installed
/// @param arg_len the length of input args
/// @param ... args
/// @return <0 represent occur some error
int frmodel_install_hook(int arg_len, ...);

/// @brief Hook: when model is uninstalled
/// @param arg_len the length of input args
/// @param ... args
/// @return <0 represent occur some error
int frmodel_uninstall_hook(int arg_len, ...);

/// @brief get the state of plant model
/// @param xu   the state matrix of current model
/// @param xdot the derivative of the state matrix
/// @return <0 represent occur some error
int frmodel_get_state(double *xu, double *xdot);

#endif // FR_MODEL_FFI_H