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

Tensor *create_tensor(int nDimension, int *nPoints);
void free_tensor(Tensor *tensor);

/*********************************************************************************
 indexVector contains the co-ordinate of a point in the ndimensional grid
 the indices along each axis are assumed to begin from zero
 *********************************************************************************/
int get_lin_index(int *indexVector, TensorInfo info);

/*******************************************/
/*    Creation of integer vector 	   */
/*******************************************/
int *create_intvector(int n);

/*********************************************/
/* 	Create a double Vector		     */
/*********************************************/
double *create_doublevector(int n);

/*******************************************/
/*    Creation of integer MATRIX 	   */
/*******************************************/
int **create_intmatrix(int n, int m);

/*********************************************/
/* 	Create a double MATRIX		     */
/*********************************************/
double **create_doublematrix(int n, int m);

/*********************************************/
/*  	Free integer matrix			  */
/*********************************************/
void free_intmatrix(int **mat, int n, int m);

/*********************************************/
/*   	Free double matrix			  */
/*********************************************/
void free_doublematrix(double **mat, int n, int m);

int fix(double in);
int sign(double in);
void atmos(double alt, double vt, double *coeff);
void accels(double *state, double *xdot, double *y);

#endif // UTILS_H