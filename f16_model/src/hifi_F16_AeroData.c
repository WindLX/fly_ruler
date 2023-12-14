#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "mexndinterp.h"
#include "fly_ruler_ffi.h"
#include "hifi_F16_AeroData.h"

#define DATA_LEN 44
#define GET_BIT(num, pos) ((num >> pos) & 1)

#define GET_COEFF_ALPHA(axisIndex, hifiIndex) \
	double targetData[1] = {alpha};           \
	return interpn(axisData[axisIndex], hifiData[hifiIndex], targetData)

#define GET_COEFF2(axisIndex, hifiIndex)  \
	double targetData[2] = {alpha, beta}; \
	return interpn(axisData[axisIndex], hifiData[hifiIndex], targetData)

#define GET_COEFF3(axisIndex, hifiIndex)        \
	double targetData[3] = {alpha, beta, dele}; \
	return interpn(axisData[axisIndex], hifiData[hifiIndex], targetData)

static Tensor **hifiData;
static double ***axisData;
static char *dataDir;

enum AxisDataIndex
{
	ALPHA1 = 0,
	ALPHA2,
	DH1,
	ALPHA1_BETA1,
	ALPHA2_BETA1,
	ALPHA1_BETA1_DH1,
	ALPHA1_BETA1_DH2
};

enum HifiDataIndex
{
	CL0120 = 0,
	CL0620,
	CL0720,
	CL0820,
	CL0920,
	CL1220,
	CL1320,
	CL1520,
	CL1620,
	CL9999,
	CM0120,
	CM0820,
	CM1020,
	CM1120,
	CM1420,
	CM9999,
	CN0120,
	CN0620,
	CN0720,
	CN0820,
	CN0920,
	CN1220,
	CN1320,
	CN1520,
	CN1620,
	CN9999,
	CX0120,
	CX0820,
	CX1120,
	CX1420,
	CY0320,
	CY0620,
	CY0720,
	CY0820,
	CY0920,
	CY1220,
	CY1320,
	CY1520,
	CY1620,
	CZ0120,
	CZ0820,
	CZ1120,
	CZ1420
};

double *loadAxisData(char *fileName, int len)
{
	FILE *fp = fopen(fileName, "r");
	int i;
	double buffer;
	char errorMsg[50];

	if (fp == NULL)
	{
		sprintf(errorMsg, "Can't find file %s", fileName);
		fclose(fp);
		log(errorMsg, ERROR);
		return NULL;
	}

	double *data = doubleVector(len);

	for (i = 0; i < len; i++)
	{
		fscanf(fp, "%lf", &buffer);
		data[i] = buffer;
	}
	fclose(fp);
	return data;
}

Tensor *loadAerodynamicData(char *fileName, int nDimension, char dataNameIndex)
{
	/**
	 * dataNameIndex:
	 * 	000: ALPHA1 BETA1 DH1
	 * 	special 0b1000 for ETA_DH1_brett
	 */
	double buffer = 0.0;
	char filePath[100];
	char errorMsg[100];
	int fileSize = 0;
	int *nPoints = (int *)malloc(nDimension * sizeof(int));

	if (nDimension > 0)
	{
		if (GET_BIT(dataNameIndex, 2) == 0)
		{
			nPoints[0] = 20;
		}
		else if (GET_BIT(dataNameIndex, 2) == 1)
		{
			nPoints[0] = 14;
		}
		else if (nDimension == 1 && GET_BIT(dataNameIndex, 3) == 1)
		{
			nPoints[0] = 5;
		}
		else
		{
			free(nPoints);
			sprintf(errorMsg, "Invalid dataNameIndex");
			log(errorMsg, ERROR);
			return NULL;
		}
		fileSize = nPoints[0];

		if (nDimension > 1)
		{
			nPoints[1] = 19;
			fileSize *= nPoints[1];

			if (nDimension == 3)
			{
				if (GET_BIT(dataNameIndex, 0) == 0)
				{
					nPoints[2] = 5;
				}
				else if (GET_BIT(dataNameIndex, 0) == 1)
				{
					nPoints[2] = 3;
				}
				else
				{
					free(nPoints);
					sprintf(errorMsg, "Invalid dataNameIndex");
					log(errorMsg, ERROR);
					return NULL;
				}
				fileSize *= nPoints[2];
			}
		}
	}

	Tensor *tensor = createTensor(nDimension, nPoints);
	free(nPoints);

	sprintf(filePath, "%s/%s", dataDir, fileName);
	FILE *fp = fopen(filePath, "r");
	if (fp == (FILE *)NULL)
	{
		freeTensor(tensor);
		fclose(fp);
		sprintf(errorMsg, "Can't find file %s", fileName);
		log(errorMsg, ERROR);
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

double **getAxisData(Tensor **hifiData, int nDimension, char dataNameIndex)
{
	double **axisData = (double **)malloc(nDimension * sizeof(double *));
	char errorMsg[100];
	if (nDimension > 0)
	{
		if (GET_BIT(dataNameIndex, 2) == 0)
		{
			axisData[0] = hifiData[0]->data;
		}
		else if (GET_BIT(dataNameIndex, 2) == 1)
		{
			axisData[0] = hifiData[1]->data;
		}
		else if (nDimension == 1 && GET_BIT(dataNameIndex, 3) == 1)
		{
			axisData[0] = hifiData[3]->data;
		}
		else
		{
			free(axisData);
			sprintf(errorMsg, "Invalid dataNameIndex");
			log(errorMsg, ERROR);
			return NULL;
		}

		if (nDimension > 1)
		{
			axisData[1] = hifiData[2]->data;

			if (nDimension == 3)
			{
				if (GET_BIT(dataNameIndex, 0) == 0)
				{
					axisData[2] = hifiData[3]->data;
				}
				else if (GET_BIT(dataNameIndex, 0) == 1)
				{
					axisData[2] = hifiData[4]->data;
				}
				else
				{
					free(axisData);
					sprintf(errorMsg, "Invalid dataNameIndex");
					log(errorMsg, ERROR);
					return NULL;
				}
			}
		}
	}
	return axisData;
}

void initHifiData()
{
	hifiData = (Tensor **)malloc(sizeof(Tensor *) * DATA_LEN);
	hifiData[0] = loadAerodynamicData("CL0120_ALPHA1_BETA1_DH2_601.dat", 3, 0b001);
	hifiData[1] = loadAerodynamicData("CL0620_ALPHA1_BETA1_604.dat", 2, 0b000);
	hifiData[2] = loadAerodynamicData("CL0720_ALPHA1_BETA1_603.dat", 2, 0b000);
	hifiData[3] = loadAerodynamicData("CL0820_ALPHA2_BETA1_602.dat", 2, 0b100);
	hifiData[4] = loadAerodynamicData("CL0920_ALPHA2_BETA1_605.dat", 2, 0b100);
	hifiData[5] = loadAerodynamicData("CL1220_ALPHA1_608.dat", 1, 0b000);
	hifiData[6] = loadAerodynamicData("CL1320_ALPHA1_606.dat", 1, 0b000);
	hifiData[7] = loadAerodynamicData("CL1520_ALPHA2_609.dat", 1, 0b100);
	hifiData[8] = loadAerodynamicData("CL1620_ALPHA2_607.dat", 1, 0b100);
	hifiData[9] = loadAerodynamicData("CL9999_ALPHA1_brett.dat", 1, 0b000);
	hifiData[10] = loadAerodynamicData("CM0120_ALPHA1_BETA1_DH1_101.dat", 3, 0b000);
	hifiData[11] = loadAerodynamicData("CM0820_ALPHA2_BETA1_102.dat", 2, 0b100);
	hifiData[12] = loadAerodynamicData("CM1020_ALPHA1_103.dat", 1, 0b000);
	hifiData[13] = loadAerodynamicData("CM1120_ALPHA1_104.dat", 1, 0b000);
	hifiData[14] = loadAerodynamicData("CM1420_ALPHA2_105.dat", 1, 0b100);
	hifiData[15] = loadAerodynamicData("CM9999_ALPHA1_brett.dat", 1, 0b000);
	hifiData[16] = loadAerodynamicData("CN0120_ALPHA1_BETA1_DH2_501.dat", 3, 0b001);
	hifiData[17] = loadAerodynamicData("CN0620_ALPHA1_BETA1_504.dat", 2, 0b000);
	hifiData[18] = loadAerodynamicData("CN0720_ALPHA1_BETA1_503.dat", 2, 0b000);
	hifiData[19] = loadAerodynamicData("CN0820_ALPHA2_BETA1_502.dat", 2, 0b100);
	hifiData[20] = loadAerodynamicData("CN0920_ALPHA2_BETA1_505.dat", 2, 0b100);
	hifiData[21] = loadAerodynamicData("CN1220_ALPHA1_508.dat", 1, 0b000);
	hifiData[22] = loadAerodynamicData("CN1320_ALPHA1_506.dat", 1, 0b000);
	hifiData[23] = loadAerodynamicData("CN1520_ALPHA2_509.dat", 1, 0b100);
	hifiData[24] = loadAerodynamicData("CN1620_ALPHA2_507.dat", 1, 0b100);
	hifiData[25] = loadAerodynamicData("CN9999_ALPHA1_brett.dat", 1, 0b000);
	hifiData[26] = loadAerodynamicData("CX0120_ALPHA1_BETA1_DH1_201.dat", 3, 0b000);
	hifiData[27] = loadAerodynamicData("CX0820_ALPHA2_BETA1_202.dat", 2, 0b100);
	hifiData[28] = loadAerodynamicData("CX1120_ALPHA1_204.dat", 1, 0b000);
	hifiData[29] = loadAerodynamicData("CX1420_ALPHA2_205.dat", 1, 0b100);
	hifiData[30] = loadAerodynamicData("CY0320_ALPHA1_BETA1_401.dat", 2, 0b000);
	hifiData[31] = loadAerodynamicData("CY0620_ALPHA1_BETA1_403.dat", 2, 0b000);
	hifiData[32] = loadAerodynamicData("CY0720_ALPHA1_BETA1_405.dat", 2, 0b000);
	hifiData[33] = loadAerodynamicData("CY0820_ALPHA2_BETA1_402.dat", 2, 0b100);
	hifiData[34] = loadAerodynamicData("CY0920_ALPHA2_BETA1_404.dat", 2, 0b100);
	hifiData[35] = loadAerodynamicData("CY1220_ALPHA1_408.dat", 1, 0b000);
	hifiData[36] = loadAerodynamicData("CY1320_ALPHA1_406.dat", 1, 0b000);
	hifiData[37] = loadAerodynamicData("CY1520_ALPHA2_409.dat", 1, 0b100);
	hifiData[38] = loadAerodynamicData("CY1620_ALPHA2_407.dat", 1, 0b100);
	hifiData[39] = loadAerodynamicData("CZ0120_ALPHA1_BETA1_DH1_301.dat", 3, 0b000);
	hifiData[40] = loadAerodynamicData("CZ0820_ALPHA2_BETA1_302.dat", 2, 0b100);
	hifiData[41] = loadAerodynamicData("CZ1120_ALPHA1_304.dat", 1, 0b000);
	hifiData[42] = loadAerodynamicData("CZ1420_ALPHA2_305.dat", 1, 0b100);
	hifiData[43] = loadAerodynamicData("ETA_DH1_brett.dat", 1, 0b1000);

	return hifiData;
}

void freeHifiData()
{
	for (int i = 0; i < DATA_LEN; i++)
	{
		freeTensor(hifiData[i]);
	}
	free(hifiData);
}

void initAxisData()
{
	double *alpha1 = loadAxisData("ALPHA1.dat", 20);
	double *alpha2 = loadAxisData("ALPHA2.dat", 14);
	double *beta1 = loadAxisData("BETA1.dat", 19);
	double *dh1 = loadAxisData("DH1.dat", 5);
	double *dh2 = loadAxisData("DH2.dat", 3);

	// alpha1
	double **axisSet0 = (double **)malloc(sizeof(double *) * 1);
	axisSet0[0] = alpha1;

	// alpha2
	double **axisSet1 = (double **)malloc(sizeof(double *) * 1);
	axisSet1[0] = alpha2;

	// dh1
	double **axisSet2 = (double **)malloc(sizeof(double *) * 1);
	axisSet2[0] = dh1;

	// alpha1 beta1
	double **axisSet3 = (double **)malloc(sizeof(double *) * 2);
	axisSet3[0] = alpha1;
	axisSet3[1] = beta1;

	// alpha2 beta1
	double **axisSet4 = (double **)malloc(sizeof(double *) * 2);
	axisSet4[0] = alpha2;
	axisSet4[1] = beta1;

	// alpha1 beta1 dh1
	double **axisSet5 = (double **)malloc(sizeof(double *) * 3);
	axisSet5[0] = alpha1;
	axisSet5[1] = beta1;
	axisSet5[2] = dh1;

	// alpha1 beta1 dh2
	double **axisSet6 = (double **)malloc(sizeof(double *) * 3);
	axisSet6[0] = alpha1;
	axisSet6[1] = beta1;
	axisSet6[2] = dh2;

	axisData = (double ***)malloc(sizeof(double **) * 7);
	axisData[0] = axisSet0;
	axisData[1] = axisSet2;
	axisData[2] = axisSet3;
	axisData[3] = axisSet3;
	axisData[4] = axisSet4;
	axisData[5] = axisSet5;
	axisData[6] = axisSet6;
}

void freeAxisData()
{
	free(axisData[0][0]);
	free(axisData[1][0]);
	free(axisData[2][0]);
	free(axisData[3][1]);
	free(axisData[6][2]);
	for (int i = 0; i < 7; i++)
	{
		free(axisData[i]);
	}
	free(axisData);
}

void setDataDir(char *dir)
{
	dataDir = dir;
}

double _Cx(double alpha, double beta, double dele)
{
	GET_COEFF3(ALPHA1_BETA1_DH1, CX0120);
}

double _Cz(double alpha, double beta, double dele)
{
	GET_COEFF3(ALPHA1_BETA1_DH1, CZ0120);
}

double _Cm(double alpha, double beta, double dele)
{
	GET_COEFF3(ALPHA1_BETA1_DH1, CM0120);
}

double _Cy(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CY0320);
}

double _Cn(double alpha, double beta, double dele)
{
	GET_COEFF3(ALPHA1_BETA1_DH2, CN0120);
}

double _Cl(double alpha, double beta, double dele)
{
	GET_COEFF3(ALPHA1_BETA1_DH2, CL0120);
}

double _Cx_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CX0820);
}

double _Cz_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CZ0820);
}

double _Cm_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CM0820);
}

double _Cy_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CY0820);
}

double _Cn_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CN0820);
}

double _Cl_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CL0820);
}

double _CXq(double alpha)
{
	GET_COEFF(ALPHA1, CX1120);
}

double _CZq(double alpha)
{
	GET_COEFF(ALPHA1, CZ1120);
}

double _CMq(double alpha)
{
	GET_COEFF(ALPHA1, CM1120);
}

double _CYp(double alpha)
{
	GET_COEFF(ALPHA1, CY1220);
}

double _CYr(double alpha)
{
	GET_COEFF(ALPHA1, CY1320);
}

double _CNr(double alpha)
{
	GET_COEFF(ALPHA1, CN1320);
}

double _CNp(double alpha)
{
	GET_COEFF(ALPHA1, CN1220);
}

double _CLp(double alpha)
{
	GET_COEFF(ALPHA1, CL1220);
}

double _CLr(double alpha)
{
	GET_COEFF(ALPHA1, CL1320);
}

double _delta_CXq_lef(double alpha)
{
	GET_COEFF(ALPHA2, CX1420);
}

double _delta_CYr_lef(double alpha)
{
	GET_COEFF(ALPHA2, CY1620);
}

double _delta_CYp_lef(double alpha)
{
	GET_COEFF(ALPHA2, CY1520);
}

double _delta_CZq_lef(double alpha)
{
	GET_COEFF(ALPHA2, CZ1420);
}

double _delta_CLr_lef(double alpha)
{
	GET_COEFF(ALPHA2, CL1620);
}

double _delta_CLp_lef(double alpha)
{
	GET_COEFF(ALPHA2, CL1520);
}

double _delta_CMq_lef(double alpha)
{
	GET_COEFF(ALPHA2, CM1420);
}

double _delta_CNr_lef(double alpha)
{
	GET_COEFF(ALPHA2, CN1620);
}

double _delta_CNp_lef(double alpha)
{
	GET_COEFF(ALPHA2, CN1520);
}

double _Cy_r30(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CY0720);
}

double _Cn_r30(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CN0720);
}

double _Cl_r30(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CL0720);
}

double _Cy_a20(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CY0620);
}

double _Cy_a20_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CY0920);
}

double _Cn_a20(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CN0620);
}

double _Cn_a20_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CN0920);
}

double _Cl_a20(double alpha, double beta)
{
	GET_COEFF2(ALPHA1_BETA1, CL0620);
}

double _Cl_a20_lef(double alpha, double beta)
{
	GET_COEFF2(ALPHA2_BETA1, CL0920);
}

double _delta_CNbeta(double alpha)
{
	GET_COEFF(ALPHA1, CN9999);
}

double _delta_CLbeta(double alpha)
{
	GET_COEFF(ALPHA1, CL9999);
}

double _delta_Cm(double alpha)
{
	GET_COEFF(ALPHA1, CM9999);
}

double _eta_el(double el)
{
	double targetData[1] = {el};
	return interpn(axisData[DH1], hifiData[43], targetData);
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
