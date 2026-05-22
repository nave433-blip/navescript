#include <node.h>
#include <iostream>

namespace nvs_addon {
    using v8::FunctionCallbackInfo;
    using v8::Isolate;
    using v8::Local;
    using v8::Object;
    using v8::String;
    using v8::Value;

    void Run(const FunctionCallbackInfo<Value>& args) {
        Isolate* isolate = args.GetIsolate();
        v8::String::Utf8Value code(isolate, args[0]);
        std::cout << "Navescript executing: " << *code << std::endl;
    }

    void Init(Local<Object> exports) {
        NODE_SET_METHOD(exports, "run", Run);
    }

    NODE_MODULE(nvs_runtime, Init)
}
