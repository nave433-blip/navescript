(component
  ;; Nλvescript Component: nasm_test
  ;; Targeted World: cli
  (import "wasi:filesystem/types@0.2.0" (instance $fs-types))
  (import "wasi:cli/stdout@0.2.0" (instance $stdout))
  (core module $base
    (import "wasi:cli/stdout@0.2.0" "get-stdout" (func $get-stdout (result i32)))
    (memory (export "memory") 1)
    (func (export "run")
      ;; log: NASM assembly module parsed from JSON wrapper
    )
  )
  (core instance $i (instantiate $base))
  (export "run" (func (canon lift (core func $i "run"))))
)