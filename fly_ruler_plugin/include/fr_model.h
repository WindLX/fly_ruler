#ifndef FR_MODEL_H
#define FR_MODEL_H

#define G 32.17

typedef struct
{
    double npos;
    double epos;
    double altitude;
    double phi;
    double theta;
    double psi;
    double velocity;
    double alpha;
    double beta;
    double p;
    double q;
    double r;
} State;

typedef struct
{
    double thrust;
    double elevator;
    double aileron;
    double rudder;
} Control;

typedef struct
{
    double c_x;
    double c_z;
    double c_m;
    double c_y;
    double c_n;
    double c_l;
} C;

typedef struct
{
    double m;
    double b;
    double s;
    double c_bar;
    double x_cg_r;
    double x_cg;
    double h_eng;
    double j_y;
    double j_xz;
    double j_z;
    double j_x;
} PlantConstants;

/// @brief load constants of this plant
/// @param constants
/// @return <0 represent occur some error
int frmodel_load_constants(
    PlantConstants *constants);

/// @brief get the air data coeff of the plant
/// @param state    the state vector of current model
/// @param control  the control vector
/// @param d_lef    the delta of leading edge flap
/// @param c        the air data under this condition
/// @return <0 represent occur some error
int frmodel_step(
    const State *state,
    const Control *control,
    double d_lef,
    C *c);

#endif // FR_MODEL_H