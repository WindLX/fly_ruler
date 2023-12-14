#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include <math.h>
#include "mexndinterp.h"

/**
 * 找到目标点被围成的超立方体网格的顶点索引
 */
int **getHyperCube(double **X, double *V, TensorInfo info)
{
	int **indexMatrix = intMatrix(info.nDimension, 2);
	/* indexMatrix[i][0] => Lower, ...[1]=>Higher */
	int i, j;
	int indexMax;
	double x, xmax, xmin;

	for (i = 0; i < info.nDimension; i++)
	{
		indexMax = info.nPoints[i]; /* Get the total # of points in this dimension */
		xmax = X[i][indexMax - 1];	/* Get the upper bound along this axis */
		xmin = X[i][0];				/* Get the lower bound along this axis */

		/****************************************************************************
			It has been assumed that the gridpoints are monotonically increasing
			the zero index is the minimum and the max-1 is the maximum.
		*****************************************************************************/

		/****************************************************************************
				Get the ith component in the vector V, the point at which we want to
				interpolate
		****************************************************************************/
		x = V[i];

		/* Check to see if this point is within the bound */
		if (x < xmin || x > xmax)
		{
			freeIntMat(indexMatrix, info.nDimension, 2);
			logError("Point lies out data grid (in getHyperCube)");
			return NULL;
		}
		else
		{
			for (j = 0; j < indexMax - 1; j++)
			{
				if (x == X[i][j])
				{
					indexMatrix[i][0] = indexMatrix[i][1] = j;
					break;
				}
				if (x == X[i][j + 1])
				{
					indexMatrix[i][0] = indexMatrix[i][1] = j + 1;
					break;
				}
				if (x > X[i][j] && x < X[i][j + 1])
				{
					indexMatrix[i][0] = j;
					indexMatrix[i][1] = j + 1;
					break;
				}
			}
		}
	}
	return (indexMatrix);
}

/**
 * 线性插值
 */
double linearInterpolate(double *T, double *V, double **X, TensorInfo info)
{
	int m, i, j, k, nVertices;
	double *oldT, *newT;
	int mask, val;
	int n = info.nDimension;
	int *indexVector = intVector(n);
	int index1, index2;
	double f1, f2, lambda, result;
	int dimNum;

	nVertices = 1 << n;

	oldT = doubleVector(nVertices);
	for (i = 0; i < nVertices; i++)
		oldT[i] = T[i];

	dimNum = 0;
	while (n > 0)
	{
		m = n - 1;
		nVertices = (1 << m);
		newT = doubleVector(nVertices);
		for (i = 0; i < nVertices; i++)
		{
			for (j = 0; j < m; j++)
			{
				mask = (1 << j);
				indexVector[j] = (mask & i) >> j;
			} /*End of for j*/
			index1 = 0;
			index2 = 0;
			for (j = 0; j < m; j++)
			{
				index1 = index1 + (1 << (j + 1)) * indexVector[j];
				index2 = index2 + (1 << j) * indexVector[j];
			} /*End of for j*/
			f1 = oldT[index1];
			f2 = oldT[index1 + 1];
			if (X[dimNum][0] != X[dimNum][1])
			{
				lambda = (V[dimNum] - X[dimNum][0]) / (X[dimNum][1] - X[dimNum][0]);
				newT[index2] = lambda * f2 + (1 - lambda) * f1;
			}
			else
				newT[index2] = f1;
		} /*End of for i*/
		free(oldT);
		oldT = doubleVector(nVertices);
		for (i = 0; i < nVertices; i++)
			oldT[i] = newT[i];
		free(newT);
		n = m;
		dimNum++;
	} /* End of while*/
	result = oldT[0];
	free(oldT);
	free(indexVector);
	return (result);
}

double interpn(double **X, Tensor *Y, double *x)
{
	double **xPoint, *T;
	double result;

	int i, j, high, low, counter;
	int mask, val, P, index, nVertices, nDimension;
	int **indexMatrix, *indexVector;

	indexVector = intVector(Y->info->nDimension);
	xPoint = doubleMatrix(Y->info->nDimension, 2);

	/* Get the indices of the hypercube containing the point in argument */
	indexMatrix = getHyperCube(X, x, *(Y->info));
	if (indexMatrix == NULL)
	{
		free(indexVector);
		freeDoubleMat(xPoint, nDimension, 2);
		return NAN;
	}

	nVertices = (1 << Y->info->nDimension);
	T = doubleVector(nVertices);

	nDimension = Y->info->nDimension;

	/* Get the co-ordinates of the hyper cube */
	for (i = 0; i < nDimension; i++)
	{
		low = indexMatrix[i][0];
		high = indexMatrix[i][1];
		xPoint[i][0] = X[i][low];
		xPoint[i][1] = X[i][high];
	}

	for (i = 0; i < nVertices; i++)
	{
		for (j = 0; j < nDimension; j++)
		{
			mask = 1 << j;
			val = (mask & i) >> j;
			indexVector[j] = indexMatrix[j][val];
		}
		index = getLinIndex(indexVector, *(Y->info));
		T[i] = Y->data[index];
	}
	result = linearInterpolate(T, x, xPoint, *(Y->info));
	free(indexVector);
	free(T);
	freeIntMat(indexMatrix, nDimension, 2);
	freeDoubleMat(xPoint, nDimension, 2);
	return (result);
}
