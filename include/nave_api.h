#include <stdio.h>

#ifndef NAVE_API_H
#define NAVE_API_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    const char* name;
    void* handle;
} NaveContext;

// Initialize Nλvescript Runtime
NaveContext* nave_init();

// Execute a Nλvescript module
int nave_run(NaveContext* ctx, const char* module_path);

// Transmute data from Nλvescript to Host
const char* nave_get_result(NaveContext* ctx, const char* var_name);

void nave_free(NaveContext* ctx);

#ifdef __cplusplus
}
#endif

#endif
