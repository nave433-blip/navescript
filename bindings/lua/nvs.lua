-- NaveScript Lua FFI Binding
local ffi = require("ffi")
ffi.cdef[[
    void* nvs_create_runtime();
    void nvs_run(void* handle, const char* code);
]]

local nvs = ffi.load("nvs_runtime")

local NvsRuntime = {}
NvsRuntime.__index = NvsRuntime

function NvsRuntime.new()
    local self = setmetatable({}, NvsRuntime)
    self.handle = nvs.nvs_create_runtime()
    return self
end

function NvsRuntime:run(code)
    nvs.nvs_run(self.handle, code)
end

return NvsRuntime
