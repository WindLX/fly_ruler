#ifndef FR_CONTROLLER_H
#define FR_CONTROLLER_H

#include <stdarg.h>

/// @brief get the state of plant model
/// @param xu   the state matrix of current model
/// @param xdot the derivative of the state matrix
/// @return <0 represent occur some error
int frcontroller_get_state(double *xu, double *xdot);

#endif // FR_CONTROLLER_H