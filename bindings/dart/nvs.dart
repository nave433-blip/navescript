// NaveScript Dart FFI Binding
import 'dart:ffi' as ffi;
import 'package:ffi/ffi.dart';

typedef nvs_create_runtime_func = ffi.Pointer<ffi.Void> Function();
typedef nvs_run_func = ffi.Void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<Utf8>);

class NvsRuntime {
  late ffi.Pointer<ffi.Void> _handle;
  late ffi.DynamicLibrary _lib;
  late ffi.Function _nvs_run;

  NvsRuntime() {
    _lib = ffi.DynamicLibrary.open("libnvs_runtime.so");
    final create = _lib.lookupFunction<nvs_create_runtime_func, nvs_create_runtime_func>("nvs_create_runtime");
    _nvs_run = _lib.lookupFunction<nvs_run_func, nvs_run_func>("nvs_run");
    _handle = create();
  }

  void run(String code) {
    final cCode = code.toNativeUtf8();
    _nvs_run(_handle, cCode);
    calloc.free(cCode);
  }
}
