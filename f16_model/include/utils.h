#ifndef UTILS_H
#define UTILS_H

typedef struct
{
    int nDimension; /* Number of dimensions*/
    int *nPoints;   /* number of points along each direction */
} TensorInfo;

typedef struct
{
    TensorInfo *info; /* TensorInfo */
    double *data;     /* data */
} Tensor;

Tensor *createTensor(int nDimension, int *nPoints);
void freeTensor(Tensor *tensor);

/*********************************************************************************
 indexVector contains the co-ordinate of a point in the ndimensional grid
 the indices along each axis are assumed to begin from zero
 *********************************************************************************/
int getLinIndex(int *indexVector, TensorInfo info);

/*******************************************/
/*    Creation of integer vector 	   */
/*******************************************/
int *intVector(int n);

/*********************************************/
/* 	Create a double Vector		     */
/*********************************************/
double *doubleVector(int n);

/*******************************************/
/*    Creation of integer MATRIX 	   */
/*******************************************/
int **intMatrix(int n, int m);

/*********************************************/
/* 	Create a double MATRIX		     */
/*********************************************/
double **doubleMatrix(int n, int m);

/*********************************************/
/*  	Free integer matrix			  */
/*********************************************/
void freeIntMat(int **mat, int n, int m);

/*********************************************/
/*   	Free double matrix			  */
/*********************************************/
void freeDoubleMat(double **mat, int n, int m);

int fix(double in);
int sign(double in);
void atmos(double alt, double vt, double *coeff);
void accels(double *state, double *xdot, double *y);

#endif // UTILS_H