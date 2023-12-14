#include "fly_ruler_ffi.h"

void set_create_int_vector_callback(CreateIntVectorCallback cb)
{
    create_int_vector = cb;
}

void set_free_int_vector_callback(FreeIntVectorCallback cb)
{
    free_int_vector = cb;
}

void set_create_double_vector_callback(CreateDoubleVectorCallback cb)
{
    create_double_vector = cb;
}

void set_free_double_vector_callback(FreeDoubleVectorCallback cb)
{
    free_double_vector = cb;
}

void set_create_int_matrix_callback(CreateIntMatrixCallback cb)
{
    create_int_matrix = cb;
}

void set_free_int_matrix_callback(FreeIntMatrixCallBack cb)
{
    free_int_matrix = cb;
}

void set_create_double_matrix_callback(CreateDoubleMatrixCallback cb)
{
    create_double_matrix = cb;
}
void set_free_double_matrix_callback(FreeDoubleMatrixCallback cb)
{
    free_double_matrix = cb;
}

void set_error_callback(ErrorCallback cb)
{
    logError = cb;
}