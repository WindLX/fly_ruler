#include <stdio.h>
#include "test_utils.h"

int frsys_step()
{
    printf("[INFO] f16 model test step\n");

    double state[12] = {
        // north_position east_position altitude
        0, 0, 15000,
        // orientation angles in rad
        0, 0.0790758040827099, 0,
        // total_velocity attack_angle sideslip_angle
        500, 0.0790758040827099, 0,
        // roll pitch yaw rate
        0, 0, 0};

    double control[4] = {
        // thrust
        2109.41286903712,
        // Elevator setting in degrees
        -2.24414978017729,
        // Ailerons mex setting in degrees
        -0.0935778861396136,
        // Rudder setting in degrees
        0.0944687551889544};

    // Leading edge flap setting in degrees
    double d_lef = 6.28161378774449;

    double state_dot[12] = {0};
    double state_extend[6] = {0};

    frmodel_step(state, control, d_lef, state_dot, state_extend);

    for (int i = 0; i < 13; i++)
    {
        printf("[INFO] state_dot[%d] = %f\n", i, state_dot[i]);
    }
    for (int i = 0; i < 7; i++)
    {
        printf("[INFO] state_extend[%d] = %f\n", i, state_extend[i]);
    }
    return 0;
}

int main()
{
    int r = fr_main(frsys_step);
    return r;
}