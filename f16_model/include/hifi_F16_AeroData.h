#ifndef HIFI_F16_AERODATA_H
#define HIFI_F16_AERODATA_H

#include "utils.h"

#define DATA_LEN 10

Tensor *loadAxisData(char *fileName, int len);
Tensor *loadAerodynamicData(char *fileName, int nDimension, char **dataName);

double _Cx(double alpha, double beta, double dele);
double _Cz(double alpha, double beta, double dele);
double _Cm(double alpha, double beta, double dele);
double _Cy(double alpha, double beta);
double _Cn(double alpha, double beta, double dele);
double _Cl(double alpha, double beta, double dele);
double _Cx_lef(double alpha, double beta);
double _Cz_lef(double alpha, double beta);
double _Cm_lef(double alpha, double beta);
double _Cy_lef(double alpha, double beta);
double _Cn_lef(double alpha, double beta);
double _Cl_lef(double alpha, double beta);
double _CXq(double alpha);
double _CZq(double alpha);
double _CMq(double alpha);
double _CYp(double alpha);
double _CYr(double alpha);
double _CNr(double alpha);
double _CNp(double alpha);
double _CLp(double alpha);
double _CLr(double alpha);
double _delta_CXq_lef(double alpha);
double _delta_CYr_lef(double alpha);
double _delta_CYp_lef(double alpha);
double _delta_CZq_lef(double alpha);
double _delta_CLr_lef(double alpha);
double _delta_CLp_lef(double alpha);
double _delta_CMq_lef(double alpha);
double _delta_CNr_lef(double alpha);
double _delta_CNp_lef(double alpha);
double _Cy_r30(double alpha, double beta);
double _Cn_r30(double alpha, double beta);
double _Cl_r30(double alpha, double beta);
double _Cy_a20(double alpha, double beta);
double _Cy_a20_lef(double alpha, double beta);
double _Cn_a20(double alpha, double beta);
double _Cn_a20_lef(double alpha, double beta);
double _Cl_a20(double alpha, double beta);
double _Cl_a20_lef(double alpha, double beta);
double _delta_CNbeta(double alpha);
double _delta_CLbeta(double alpha);
double _delta_Cm(double alpha);
double _eta_el(double el);
// double _delta_Cm_ds(double alpha, double el);

Tensor **initHifiData();
void freeHifiData(Tensor **data);

void hifi_C(double alpha, double beta, double el, double *retVal);
void hifi_damping(double alpha, double *retVal);
void hifi_C_lef(double alpha, double beta, double *retVal);
void hifi_damping_lef(double alpha, double *retVal);
void hifi_rudder(double alpha, double beta, double *retVal);
void hifi_ailerons(double alpha, double beta, double *retVal);
void hifi_other_coeffs(double alpha, double el, double *retVal);

#endif // HIFI_F16_AERODATA_H
