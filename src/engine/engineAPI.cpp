#include <napi.h>
#include "engine.h"

Napi::String GenerateWrapped(const Napi::CallbackInfo& info) {
    Napi::Env env = info.Env();

    // Check for correct number and type of arguments
    if (info.Length() != 1 || !info[0].IsNumber()) {
        Napi::TypeError::New(env, "Number expected").ThrowAsJavaScriptException();
        return Napi::String::New(env, "");  // Return an empty string on error
    }

    int difficulty = info[0].As<Napi::Number>().Int32Value();
    std::string result = generate(difficulty);  // Call the C++ function
    return Napi::String::New(env, result);
}

Napi::String FindNextStepWrapped(const Napi::CallbackInfo& info) {
    Napi::Env env = info.Env();

    if (info.Length() != 1 || !info[0].IsString()) {
        Napi::TypeError::New(env, "String expected").ThrowAsJavaScriptException();
        return Napi::String::New(env, "");  // Return an empty string on error
    }

    std::string grid = info[0].As<Napi::String>().Utf8Value();
    std::string result = findNextStep(grid);  // Call the C++ function
    return Napi::String::New(env, result);
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
    exports.Set(Napi::String::New(env, "generate"), Napi::Function::New(env, GenerateWrapped));
    exports.Set(Napi::String::New(env, "findNextStep"), Napi::Function::New(env, FindNextStepWrapped));
    return exports;
}

NODE_API_MODULE(tsudoku-engine, Init)
