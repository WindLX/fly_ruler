#ifndef FR_MODEL_H
#define FR_MODEL_H

/// @brief get the state of plant model
/// @param xu   the state matrix of current model
/// @param xdot the derivative of the state matrix
/// @return <0 represent occur some error
int frmodel_get_state(double *xu, double *xdot);

#endif // FR_MODEL_H