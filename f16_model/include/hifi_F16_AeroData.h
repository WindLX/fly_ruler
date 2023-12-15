#ifndef HIFI_F16_AERODATA_H
#define HIFI_F16_AERODATA_H

void set_data_dir(char *dir);
int init_hifi_data();
void free_hifi_data();
int init_axis_data();
void free_axis_data();

void hifi_C(double alpha, double beta, double el, double *retVal);
void hifi_damping(double alpha, double *retVal);
void hifi_C_lef(double alpha, double beta, double *retVal);
void hifi_damping_lef(double alpha, double *retVal);
void hifi_rudder(double alpha, double beta, double *retVal);
void hifi_ailerons(double alpha, double beta, double *retVal);
void hifi_other_coeffs(double alpha, double el, double *retVal);

#endif // HIFI_F16_AERODATA_H
