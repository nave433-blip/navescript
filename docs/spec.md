# NaveScript Language Specification

## 1. Syntax
NaveScript uses a C-like syntax with support for polyglot embedding.

### 1.1 Variables
```nvs
let x = 42;
let y = "hello";
```

### 1.2 Functions
```nvs
func add(a, b) {
    return a + b;
}
```

### 1.3 Control Flow
```nvs
if (x > 0) {
    print("Positive");
} else {
    print("Negative");
}
```
