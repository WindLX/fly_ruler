#ifndef MEXNDINTERP_H
#define MEXNDINTERP_H

#include "utils.h"

/************************************************/
/*    Get the indices of the hyper cube in the  */
/*    grid in which the point lies              */
/************************************************/
int **getHyperCube(double **X, double *V, TensorInfo info);

double linearInterpolate(double *T, double *V, double **X, TensorInfo info);
double interpn(double **X, Tensor *Y, double *x);

#endif // MEXNDINTERP_H