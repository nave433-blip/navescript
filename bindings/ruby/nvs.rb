# NaveScript Ruby Binding
require 'ffi'

module Nvs
  extend FFI::Library
  ffi_lib 'nvs_runtime'
  
  attach_function :nvs_create_runtime, [], :pointer
  attach_function :nvs_run, [:pointer, :string], :void
  
  class Runtime
    def initialize
      @handle = Nvs.nvs_create_runtime
    end
    
    def run(code)
      Nvs.nvs_run(@handle, code)
    end
  end
end
