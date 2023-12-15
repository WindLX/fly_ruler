#include <stdio.h>
#include "test_utils.h"

int frsys_step()
{
    printf("[INFO] f16 model test step\n");

    double xu[18] = {
        // north_position east_position altitude
        0, 0, 15000,
        // orientation angles in rad
        0, 0.0790758040827099, 0,
        // total_velocity attack_angle sideslip_angle
        500, 0.0790758040827099, 0,
        // roll pitch yaw rate
        0, 0, 0,
        // thrust
        2109.41286903712,
        // Elevator setting in degrees
        -2.24414978017729,
        // Ailerons mex setting in degrees
        -0.0935778861396136,
        // Rudder setting in degrees
        0.0944687551889544,
        // Leading edge flap setting in degrees
        6.28161378774449,
        // hifi model
        1};
    double xdot[18] = {0};

    frmodel_get_state(xu, xdot);

    for (int i = 0; i < 18; i++)
    {
        printf("[INFO] xdot[%d] = %f\n", i, xdot[i]);
    }
    return 0;
}

int main()
{
    int r = fr_main(frsys_step);
    return r;
}