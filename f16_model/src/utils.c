#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "utils.h"

int fix(double in)
{
    int out;

    if (in >= 0.0)
    {
        out = (int)floor(in);
    }
    else if (in < 0.0)
    {
        out = (int)ceil(in);
    }

    return out;
}

int sign(double in)
{
    int out;

    if (in > 0.0)
    {
        out = 1;
    }
    else if (in < 0.0)
    {
        out = -1;
    }
    else if (in == 0.0)
    {
        out = 0;
    }
    return out;
}

void atmos(double alt, double vt, double *coeff)
{
    double rho0 = 2.377e-3;
    double tfac, temp, rho, mach, qbar, ps;

    tfac = 1 - .703e-5 * (alt);
    temp = 519.0 * tfac;
    if (alt >= 35000.0)
    {
        temp = 390;
    }

    rho = rho0 * pow(tfac, 4.14);
    mach = (vt) / sqrt(1.4 * 1716.3 * temp);
    qbar = .5 * rho * pow(vt, 2);
    ps = 1715.0 * rho * temp;

    if (ps == 0)
    {
        ps = 1715;
    }

    coeff[0] = mach;
    coeff[1] = qbar;
    coeff[2] = ps;
}

void accels(const State *state,
            const State *state_dot,
            double *y)
{
#define GRAV 32.174

    double sina, cosa, sinb, cosb;
    double vel_u, vel_v, vel_w;
    double u_dot, v_dot, w_dot;
    double nx_cg, ny_cg, nz_cg;

    sina = sin(state->alpha);
    cosa = cos(state->alpha);
    sinb = sin(state->beta);
    cosb = cos(state->beta);
    vel_u = state->velocity * cosb * cosa;
    vel_v = state->velocity * sinb;
    vel_w = state->velocity * cosb * sina;
    u_dot = cosb * cosa * state_dot->velocity - state->velocity * sinb * cosa * state_dot->beta - state->velocity * cosb * sina * state_dot->alpha;
    v_dot = sinb * state_dot->velocity + state->velocity * cosb * state_dot->beta;
    w_dot = cosb * sina * state_dot->velocity - state->velocity * sinb * sina * state_dot->beta + state->velocity * cosb * cosa * state_dot->alpha;
    nx_cg = 1.0 / GRAV * (u_dot + state->q * vel_w - state->r * vel_v) + sin(state->theta);
    ny_cg = 1.0 / GRAV * (v_dot + state->r * vel_u - state->p * vel_w) - cos(state->theta) * sin(state->phi);
    nz_cg = -1.0 / GRAV * (w_dot + state->p * vel_v - state->q * vel_u) + cos(state->theta) * cos(state->phi);

    y[0] = nx_cg;
    y[1] = ny_cg;
    y[2] = nz_cg;
}

Tensor *create_tensor(int n_dimension, int *n_points)
{
    int length = 1;
    TensorInfo *info = (TensorInfo *)malloc(sizeof(TensorInfo));
    info->n_dimension = n_dimension;
    info->n_points = (int *)malloc(n_dimension * sizeof(int));
    memcpy(info->n_points, n_points, n_dimension * sizeof(int));
    Tensor *tensor = (Tensor *)malloc(sizeof(Tensor));
    tensor->info = info;
    for (int i = 0; i < info->n_dimension; i++)
    {
        length *= info->n_points[i];
    }
    tensor->data = (double *)malloc(length * sizeof(double));
    return (tensor);
}

void free_tensor(Tensor *tensor)
{
    free(tensor->info->n_points);
    free(tensor->info);
    free(tensor->data);
    free(tensor);
}

int get_lin_index(int *indexVector, TensorInfo info)
{
    int linIndex = 0;
    int i, j, P;
    for (i = 0; i < info.n_dimension; i++)
    {
        P = 1;
        for (j = 0; j < i; j++)
            P = P * info.n_points[j];
        linIndex += P * indexVector[i];
    }
    return (linIndex);
}

int *create_ivector(int n)
{
    int *vec = (int *)malloc(n * sizeof(int));
    return (vec);
}

double *create_dvector(int n)
{
    double *vec = (double *)malloc(n * sizeof(double));
    return (vec);
}

int **create_imatrix(int n, int m)
{
    int i;
    int **mat = (int **)malloc(n * sizeof(int *));
    for (i = 0; i < n; i++)
        mat[i] = (int *)malloc(m * sizeof(int));
    return (mat);
}

double **create_dmatrix(int n, int m)
{
    int i;
    double **mat = (double **)malloc(n * sizeof(double *));
    for (i = 0; i < n; i++)
        mat[i] = (double *)malloc(m * sizeof(double));
    return (mat);
}

void free_imatrix(int **mat, int n, int m)
{
    /*
        the column size is not used but is required only
        for debugging purpose
    */
    int i;
    for (i = 0; i < n; i++)
        free(mat[i]);
    free(mat);
}

void free_dmatrix(double **mat, int n, int m)
{
    /*
        the column size is not used but is required only
        for debugging purpose
    */
    int i;
    for (i = 0; i < n; i++)
        free(mat[i]);
    free(mat);
}