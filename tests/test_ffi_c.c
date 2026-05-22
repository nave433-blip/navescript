#include <stdio.h>
#include <stdlib.h>
#include "nave_api.h"

int main() {
    printf("--- C Host: Initializing Nλvescript Runtime ---\n");
    NaveContext* ctx = nave_init();
    if (ctx == NULL) {
        printf("Failed to init runtime\n");
        return 1;
    }

    printf("--- C Host: Running Module ---\n");
    int result = nave_run(ctx, "examples/hello.nave");
    printf("--- C Host: Result = %d ---\n", result);

    nave_free(ctx);
    printf("--- C Host: Runtime Shutdown ---\n");
    return 0;
}
