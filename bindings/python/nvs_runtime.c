#include <Python.h>
#include <stdio.h>

// This is the C-Extension for Python to call the Navescript runtime
static PyObject* nvs_run(PyObject* self, PyObject* args) {
    const char* code;
    if (!PyArg_ParseTuple(args, "s", &code)) return NULL;
    printf("Navescript executing: %s\n", code);
    Py_RETURN_NONE;
}

static PyMethodDef NvsMethods[] = {
    {"run", nvs_run, METH_VARARGS, "Execute Navescript code"},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef nvsmodule = {
    PyModuleDef_HEAD_INIT, "nvs_runtime", NULL, -1, NvsMethods
};

PyMODINIT_FUNC PyInit_nvs_runtime(void) {
    return PyModule_Create(&nvsmodule);
}
