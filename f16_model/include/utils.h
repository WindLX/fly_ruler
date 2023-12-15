#ifndef UTILS_H
#define UTILS_H

typedef struct
{
    int n_dimension; /* Number of dimensions*/
    int *n_points;   /* number of points along each direction */
} TensorInfo;

typedef struct
{
    TensorInfo *info; /* TensorInfo */
    double *data;     /* data */
} Tensor;

Tensor *create_tensor(int n_dimension, int *n_points);
void free_tensor(Tensor *tensor);

/**
 * indexVector contains the co-ordinate of a point in the ndimensional grid
 * the indices along each axis are assumed to begin from zero
 */
int get_lin_index(int *indexVector, TensorInfo info);

// Creation of integer Vector
int *create_ivector(int n);

// Create a double Vector
double *create_dvector(int n);

// Creation of integer MATRIX
int **create_imatrix(int n, int m);

// Create a double MATRIX
double **create_dmatrix(int n, int m);

// Free integer matrix
void free_imatrix(int **mat, int n, int m);

// Free double matrix
void free_dmatrix(double **mat, int n, int m);

// fix double to integer
int fix(double in);

// sign function
int sign(double in);

// Function for mach and qbar
void atmos(double alt, double vt, double *coeff);

// Calculate accelerations from states and state derivatives.
void accels(double *state, double *xdot, double *y);

#endif // UTILS_H