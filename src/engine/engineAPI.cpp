#include <napi.h>
#include "Grid.h"

Napi::String GenerateWrapped(const Napi::CallbackInfo& info) {
    Napi::Env env = info.Env();

    // Check for correct number and type of arguments
    if (info.Length() != 1 || !info[0].IsNumber()) {
        Napi::TypeError::New(env, "Number expected").ThrowAsJavaScriptException();
        return Napi::String::New(env, "");  // Return an empty string on error
    }

    int difficulty = info[0].As<Napi::Number>().Int32Value();
    Grid grid(difficulty);
    std::string result =grid.toString();  // Call the C++ function
    return Napi::String::New(env, result);
}

Napi::Buffer<uint8_t> FindNextStepWrapped(const Napi::CallbackInfo& info) {
    Napi::Env env = info.Env();
    if (info.Length() != 1 || !info[0].IsString()) {
        Napi::TypeError::New(env, "String expected").ThrowAsJavaScriptException();
        return Napi::Buffer<uint8_t>::New(env, 0);  // 返回一个空的buffer
    }

    std::string gridPattern = info[0].As<Napi::String>().Utf8Value();
    Grid grid(gridPattern);
    std::vector<uint8_t>& result = grid.nextStep();  // 调用C++函数获取引用

    // 使用结果数据创建Buffer，这里数据会被复制到Node.js的Buffer中
    return Napi::Buffer<uint8_t>::Copy(env, result.data(), result.size());
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
    exports.Set(Napi::String::New(env, "generate"), Napi::Function::New(env, GenerateWrapped));
    exports.Set(Napi::String::New(env, "findNextStep"), Napi::Function::New(env, FindNextStepWrapped));
    return exports;
}

NODE_API_MODULE(tsudoku-engine, Init)
