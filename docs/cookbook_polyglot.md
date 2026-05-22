# NaveScript Polyglot Cookbook

## Python Integration
Run Python code directly within NaveScript.

```nvs
@python {
    def greet(name):
        return f"Hello, {name}!"
    print(greet("NaveScript"))
}
```

## JavaScript Integration
Execute JavaScript using the local node/deno runtime.

```nvs
@js {
    console.log("Hello from JavaScript!");
}
```

## NASM Integration
Run low-level assembly for critical performance paths.

```nvs
@asm {
    section .text
    global _start
    _start:
        mov eax, 1
        int 0x80
}
```
