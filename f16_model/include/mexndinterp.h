#ifndef MEXNDINTERP_H
#define MEXNDINTERP_H

#include "utils.h"

// int **getHyperCube(double **axisData, double *targetData, TensorInfo info);
// double linearInterpolate(double *T, double *targetData, double **axisData, TensorInfo info);

double interpn(double **axisData, Tensor *data, double *targetData);

#endif // MEXNDINTERP_H