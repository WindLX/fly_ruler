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
} PlaneConstants;

typedef struct
{
    double thrust_cmd_limit_top;
    double thrust_cmd_limit_bottom;
    double thrust_rate_limit;
    double ele_cmd_limit_top;
    double ele_cmd_limit_bottom;
    double ele_rate_limit;
    double ail_cmd_limit_top;
    double ail_cmd_limit_bottom;
    double ail_rate_limit;
    double rud_cmd_limit_top;
    double rud_cmd_limit_bottom;
    double rud_rate_limit;
    double alpha_limit_top;
    double alpha_limit_bottom;
    double beta_limit_top;
    double beta_limit_bottom;
} ControlLimit;

/// @brief load constants of this plane
/// @param constants
/// @return <0 represent occur some err
int frmodel_load_constants(PlaneConstants *constants);

/// @brief load ctrl_limits of this plane
/// @param ctrl_limits
/// @return <0 represent occur some err
int frmodel_load_ctrl_limits(ControlLimit *ctrl_limits);

/// @brief when model is installing
/// @param argc the length of input args
/// @param argv the data of args
/// @return <0 represent occur some error
int frmodel_install(int argc, char **argv);

/// @brief when model is uninstalled
/// @return <0 represent occur some error
int frmodel_uninstall();

/// @brief get the air data coeff of the plane
/// @param state    the state vector of current model
/// @param control  the control vector
/// @param d_lef    the delta of leading edge flap
/// @param c        the air data under this condition
/// @return <0 represent occur some err
int frmodel_step(
    const State *state, const Control *control, double d_lef,
    C *c);

#endif // FR_MODEL_H