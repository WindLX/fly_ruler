#ifndef FR_MODEL_H
#define FR_MODEL_H

/// @brief get the dot of next state of the plant
/// @param state    the state vector of current model
/// @param control  the control vector
/// @param d_lef    the delta of leading edge flap
/// @param state_dot    the derivative of the state vector
/// @param state_extend  extend state value calculated by the model
/// @return <0 represent occur some error
int frmodel_step(
    double *state,
    double *control,
    double d_lef,
    double *state_dot,
    double *state_extend);

#endif // FR_MODEL_H