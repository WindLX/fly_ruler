#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "hifi_F16_AeroData.h"
#include "fly_ruler_ffi.h"

Tensor *loadAxisData(char *fileName, int len)
{
	FILE *fp = fopen(fileName, "r");
	int i;
	double buffer;
	char errorMsg[50];

	if (fp == NULL)
	{
		sprintf(errorMsg, "Can't find file %s", fileName);
		fclose(fp);
		logError(errorMsg);
		return NULL;
	}

	int lens[1] = {len};
	Tensor *tensor = createTensor(1, lens);

	for (i = 0; i < len; i++)
	{
		fscanf(fp, "%lf", &buffer);
		tensor->data[i] = buffer;
	}
	fclose(fp);
	return tensor;
}

Tensor *loadAerodynamicData(char *fileName, int nDimension, char **dataName)
{
	double buffer = 0.0;
	char errorMsg[50];
	int fileSize = 0;
	int *nPoints = (int *)malloc(nDimension * sizeof(int));

	if (nDimension > 0)
	{
		if (!strcmp(dataName[0], "ALPHA1"))
		{
			nPoints[0] = 20;
		}
		else if (!strcmp(dataName[0], "ALPHA2"))
		{
			nPoints[0] = 14;
		}
		else if (nDimension == 1 && !strcmp(dataName[0], "DH1"))
		{
			nPoints[0] = 5;
		}
		else
		{
			free(nPoints);
			sprintf(errorMsg, "Invalid dataName");
			logError(errorMsg);
			return NULL;
		}
		fileSize = nPoints[0];

		if (nDimension > 1)
		{
			nPoints[1] = 19;
			fileSize *= nPoints[1];

			if (nDimension == 3)
			{
				if (!strcmp(dataName[1], "DH1"))
				{
					nPoints[2] = 5;
				}
				else if (!strcmp(dataName[0], "DH2"))
				{
					nPoints[2] = 3;
				}
				else
				{
					free(nPoints);
					sprintf(errorMsg, "Invalid dataName");
					logError(errorMsg);
					return NULL;
				}
				fileSize *= nPoints[2];
			}
		}
	}

	Tensor *tensor = createTensor(nDimension, nPoints);
	free(nPoints);

	FILE *fp = fopen(fileName, "r");
	if (fp == (FILE *)NULL)
	{
		freeTensor(tensor);
		fclose(fp);
		sprintf(errorMsg, "Can't find file %s", fileName);
		logError(errorMsg);
		return NULL;
	}

	for (int i = 0; i < fileSize; i++)
	{
		fscanf(fp, "%lf", &buffer);
		tensor->data[i] = buffer;
	}
	fclose(fp);

	return tensor;
}

Tensor **initHifiData()
{
	Tensor **data = (Tensor **)malloc(sizeof(Tensor *) * DATA_LEN);
	data[0] = loadAxisData("ALPHA1.dat", 20);
	data[1] = loadAxisData("ALPHA2.dat", 14);
	data[2] = loadAxisData("BETA1.dat", 19);
	data[3] = loadAxisData("DH1.dat", 5);
	data[4] = loadAxisData("DH2.dat", 3);
	return data;
}

void freeHifiData(Tensor **data)
{
	for (int i = 0; i < DATA_LEN; i++)
	{
		freeTensor(data[i]);
	}
	free(data);
}

double _Cx(double alpha, double beta, double dele)
{
	int dataIndex[] = {0, 0};
	return loadCoefficient3("CX0120_ALPHA1_BETA1_DH1_201.dat", 1900, dataIndex, alpha, beta, dele, logError);
}

double _Cz(double alpha, double beta, double dele)
{
	int dataIndex[] = {0, 0};
	return loadCoefficient3("CZ0120_ALPHA1_BETA1_DH1_301.dat", 1900, dataIndex, alpha, beta, dele, logError);
}

double _Cm(double alpha, double beta, double dele)
{
	int dataIndex[] = {0, 0};
	return loadCoefficient3("CM0120_ALPHA1_BETA1_DH1_101.dat", 1900, dataIndex, alpha, beta, dele, logError);
}

double _Cy(double alpha, double beta)
{
	return loadCoefficient2("CY0320_ALPHA1_BETA1_401.dat", 380, 0, alpha, beta, logError);
}

double _Cn(double alpha, double beta, double dele)
{
	int dataIndex[] = {0, 1};
	return loadCoefficient3("CN0120_ALPHA1_BETA1_DH2_501.dat", 1140, dataIndex, alpha, beta, dele, logError);
}

double _Cl(double alpha, double beta, double dele)
{
	int dataIndex[] = {0, 1};
	return loadCoefficient3("CL0120_ALPHA1_BETA1_DH2_601.dat", 1140, dataIndex, alpha, beta, dele, logError);
}

double _Cx_lef(double alpha, double beta)
{
	return loadCoefficient2("CX0820_ALPHA2_BETA1_202.dat", 266, 1, alpha, beta, logError);
}

double _Cz_lef(double alpha, double beta)
{
	return loadCoefficient2("CZ0820_ALPHA2_BETA1_302.dat", 266, 1, alpha, beta, logError);
}

double _Cm_lef(double alpha, double beta)
{
	return loadCoefficient2("CM0820_ALPHA2_BETA1_102.dat", 266, 1, alpha, beta, logError);
}

double _Cy_lef(double alpha, double beta)
{
	return loadCoefficient2("CY0820_ALPHA2_BETA1_402.dat", 266, 1, alpha, beta, logError);
}

double _Cn_lef(double alpha, double beta)
{
	return loadCoefficient2("CN0820_ALPHA2_BETA1_502.dat", 266, 1, alpha, beta, logError);
}

double _Cl_lef(double alpha, double beta)
{
	return loadCoefficient2("CL0820_ALPHA2_BETA1_602.dat", 266, 1, alpha, beta, logError);
}

double _CXq(double alpha)
{
	return loadCoefficientAlpha("CX1120_ALPHA1_204.dat", 20, 0, alpha, logError);
}

double _CZq(double alpha)
{
	return loadCoefficientAlpha("CZ1120_ALPHA1_304.dat", 20, 0, alpha, logError);
}

double _CMq(double alpha)
{
	return loadCoefficientAlpha("CM1120_ALPHA1_104.dat", 20, 0, alpha, logError);
}

double _CYp(double alpha)
{
	return loadCoefficientAlpha("CY1220_ALPHA1_408.dat", 20, 0, alpha, logError);
}

double _CYr(double alpha)
{
	return loadCoefficientAlpha("CY1320_ALPHA1_406.dat", 20, 0, alpha, logError);
}

double _CNr(double alpha)
{
	return loadCoefficientAlpha("CN1320_ALPHA1_506.dat", 20, 0, alpha, logError);
}

double _CNp(double alpha)
{
	return loadCoefficientAlpha("CN1220_ALPHA1_508.dat", 20, 0, alpha, logError);
}

double _CLp(double alpha)
{
	return loadCoefficientAlpha("CL1220_ALPHA1_608.dat", 20, 0, alpha, logError);
}

double _CLr(double alpha)
{
	return loadCoefficientAlpha("CL1320_ALPHA1_606.dat", 20, 0, alpha, logError);
}

double _delta_CXq_lef(double alpha)
{
	return loadCoefficientAlpha("CX1420_ALPHA2_205.dat", 14, 1, alpha, logError);
}

double _delta_CYr_lef(double alpha)
{
	return loadCoefficientAlpha("CY1620_ALPHA2_407.dat", 14, 1, alpha, logError);
}

double _delta_CYp_lef(double alpha)
{
	return loadCoefficientAlpha("CY1520_ALPHA2_409.dat", 14, 1, alpha, logError);
}

double _delta_CZq_lef(double alpha)
{
	return loadCoefficientAlpha("CZ1420_ALPHA2_305.dat", 14, 1, alpha, logError);
}

double _delta_CLr_lef(double alpha)
{
	return loadCoefficientAlpha("CL1620_ALPHA2_607.dat", 14, 1, alpha, logError);
}

double _delta_CLp_lef(double alpha)
{
	return loadCoefficientAlpha("CL1520_ALPHA2_609.dat", 14, 1, alpha, logError);
}

double _delta_CMq_lef(double alpha)
{
	return loadCoefficientAlpha("CM1420_ALPHA2_105.dat", 14, 1, alpha, logError);
}

double _delta_CNr_lef(double alpha)
{
	return loadCoefficientAlpha("CN1620_ALPHA2_507.dat", 14, 1, alpha, logError);
}

double _delta_CNp_lef(double alpha)
{
	return loadCoefficientAlpha("CN1520_ALPHA2_509.dat", 14, 1, alpha, logError);
}

double _Cy_r30(double alpha, double beta)
{
	return loadCoefficient2("CY0720_ALPHA1_BETA1_405.dat", 380, 0, alpha, beta, logError);
}

double _Cn_r30(double alpha, double beta)
{
	return loadCoefficient2("CN0720_ALPHA1_BETA1_503.dat", 380, 0, alpha, beta, logError);
}

double _Cl_r30(double alpha, double beta)
{
	return loadCoefficient2("CL0720_ALPHA1_BETA1_603.dat", 380, 0, alpha, beta, logError);
}

double _Cy_a20(double alpha, double beta)
{
	return loadCoefficient2("CY0620_ALPHA1_BETA1_403.dat", 380, 0, alpha, beta, logError);
}

double _Cy_a20_lef(double alpha, double beta)
{
	return loadCoefficient2("CY0920_ALPHA2_BETA1_404.dat", 266, 1, alpha, beta, logError);
}

double _Cn_a20(double alpha, double beta)
{
	return loadCoefficient2("CN0620_ALPHA1_BETA1_504.dat", 380, 0, alpha, beta, logError);
}

double _Cn_a20_lef(double alpha, double beta)
{
	return loadCoefficient2("CN0920_ALPHA2_BETA1_505.dat", 266, 1, alpha, beta, logError);
}

double _Cl_a20(double alpha, double beta)
{
	return loadCoefficient2("CL0620_ALPHA1_BETA1_604.dat", 380, 0, alpha, beta, logError);
}

double _Cl_a20_lef(double alpha, double beta)
{
	return loadCoefficient2("CL0920_ALPHA2_BETA1_605.dat", 266, 1, alpha, beta, logError);
}

double _delta_CNbeta(double alpha)
{
	return loadCoefficientAlpha("CN9999_ALPHA1_brett.dat", 20, 0, alpha, logError);
}

double _delta_CLbeta(double alpha)
{
	return loadCoefficientAlpha("CL9999_ALPHA1_brett.dat", 20, 0, alpha, logError);
}

double _delta_Cm(double alpha)
{
	return loadCoefficientAlpha("CM9999_ALPHA1_brett.dat", 20, 0, alpha, logError);
}

double _eta_el(double el)
{
	return loadCoefficientDh("ETA_DH1_brett.dat", 5, 0, el, logError);
}

void hifi_C(double alpha, double beta, double el, double *retVal)
{
	retVal[0] = _Cx(alpha, beta, el);
	retVal[1] = _Cz(alpha, beta, el);
	retVal[2] = _Cm(alpha, beta, el);
	retVal[3] = _Cy(alpha, beta);
	retVal[4] = _Cn(alpha, beta, el);
	retVal[5] = _Cl(alpha, beta, el);
}

void hifi_damping(double alpha, double *retVal)
{
	retVal[0] = _CXq(alpha);
	retVal[1] = _CYr(alpha);
	retVal[2] = _CYp(alpha);
	retVal[3] = _CZq(alpha);
	retVal[4] = _CLr(alpha);
	retVal[5] = _CLp(alpha);
	retVal[6] = _CMq(alpha);
	retVal[7] = _CNr(alpha);
	retVal[8] = _CNp(alpha);
}

void hifi_C_lef(double alpha, double beta, double *retVal)
{
	retVal[0] = _Cx_lef(alpha, beta) - _Cx(alpha, beta, 0);
	retVal[1] = _Cz_lef(alpha, beta) - _Cz(alpha, beta, 0);
	retVal[2] = _Cm_lef(alpha, beta) - _Cm(alpha, beta, 0);
	retVal[3] = _Cy_lef(alpha, beta) - _Cy(alpha, beta);
	retVal[4] = _Cn_lef(alpha, beta) - _Cn(alpha, beta, 0);
	retVal[5] = _Cl_lef(alpha, beta) - _Cl(alpha, beta, 0);
}

void hifi_damping_lef(double alpha, double *retVal)
{
	retVal[0] = _delta_CXq_lef(alpha);
	retVal[1] = _delta_CYr_lef(alpha);
	retVal[2] = _delta_CYp_lef(alpha);
	retVal[3] = _delta_CZq_lef(alpha);
	retVal[4] = _delta_CLr_lef(alpha);
	retVal[5] = _delta_CLp_lef(alpha);
	retVal[6] = _delta_CMq_lef(alpha);
	retVal[7] = _delta_CNr_lef(alpha);
	retVal[8] = _delta_CNp_lef(alpha);
}

void hifi_rudder(double alpha, double beta, double *retVal)
{
	retVal[0] = _Cy_r30(alpha, beta) - _Cy(alpha, beta);
	retVal[1] = _Cn_r30(alpha, beta) - _Cn(alpha, beta, 0);
	retVal[2] = _Cl_r30(alpha, beta) - _Cl(alpha, beta, 0);
}

void hifi_ailerons(double alpha, double beta, double *retVal)
{
	retVal[0] = _Cy_a20(alpha, beta) - _Cy(alpha, beta);
	retVal[1] = _Cy_a20_lef(alpha, beta) - _Cy_lef(alpha, beta) - retVal[0];
	retVal[2] = _Cn_a20(alpha, beta) - _Cn(alpha, beta, 0);
	retVal[3] = _Cn_a20_lef(alpha, beta) - _Cn_lef(alpha, beta) - retVal[2];
	retVal[4] = _Cl_a20(alpha, beta) - _Cl(alpha, beta, 0);
	retVal[5] = _Cl_a20_lef(alpha, beta) - _Cl_lef(alpha, beta) - retVal[4];
}

void hifi_other_coeffs(double alpha, double el, double *retVal)
{
	retVal[0] = _delta_CNbeta(alpha);
	retVal[1] = _delta_CLbeta(alpha);
	retVal[2] = _delta_Cm(alpha);
	retVal[3] = _eta_el(el);
	retVal[4] = 0; /* ignore deep-stall regime, delta_Cm_ds = 0 */
}
