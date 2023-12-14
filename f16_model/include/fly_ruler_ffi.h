#ifndef FLY_RULER_FFI_H
#define FLY_RULER_FFI_H

typedef void (*ErrorCallback)(char *error_msg);
extern ErrorCallback logError = NULL;

void set_error_callback(ErrorCallback cb);

#endif // FLY_RULER_FFI_H