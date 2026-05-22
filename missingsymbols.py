Python 3.8.0 (v3.8.0:fa919fdf25, Oct 14 2019, 10:23:27) 
[Clang 6.0 (clang-600.0.57)] on darwin
Type "help", "copyright", "credits" or "license()" for more information.
>>> # 🔧 THE COMPLETE "EVERYTHING" REFERENCE — NAVESCRIPT v2.0

Here's the **exhaustive list** of every argument, symbol, environment variable, CLI flag, and execution parameter across **all supported languages and runtimes**.

---

## PART 1: NAVESCRIPT CLI ARGUMENTS (The Main Tool)

### Core Commands
```bash
navescript <command> [options] [arguments]
```

| Command | Description | Example |
|---------|-------------|---------|
| `run` | Execute a `.ns` or `.nave` file | `navescript run main.ns` |
| `eval` | Execute code directly from string | `navescript eval 'print("hello")'` |
| `repl` | Start interactive REPL | `navescript repl` |
| `test` | Run test suite | `navescript test --filter "http"` |
| `fmt` | Format code | `navescript fmt --write` |
| `lint` | Lint code | `navescript lint --fix` |
| `doc` | Generate documentation | `navescript doc --serve` |
| `build` | Compile to target | `navescript build --target wasm` |
| `install` | Install packages | `navescript install @stdlib/http` |
| `publish` | Publish package | `navescript publish --tag latest` |
| `init` | Create new project | `navescript init my-project` |
| `clean` | Remove build artifacts | `navescript clean` |
| `version` | Show version | `navescript version` |
| `help` | Show help | `navescript help run` |

---

### Global CLI Flags (Applicable to All Commands)

| Flag | Description | Example |
|------|-------------|---------|
| `--help`, `-h` | Show help message | `navescript run --help` |
| `--version`, `-v` | Show version | `navescript --version` |
| `--verbose`, `-V` | Verbose output (debug logs) | `navescript run --verbose` |
| `--quiet`, `-q` | Suppress non-error output | `navescript build --quiet` |
| `--color` | Force color output (always/never/auto) | `navescript test --color always` |
| `--no-color` | Disable color output | `navescript lint --no-color` |
| `--json` | Output as JSON (for tooling) | `navescript test --json` |
| `--log-level` | Set log level (trace/debug/info/warn/error) | `navescript run --log-level=debug` |
| `--log-module` | Per-module log level | `navescript run --log-module=http=debug` |
| `--log-format` | Log format (pretty/json/compact) | `navescript run --log-format=json` |

---

## PART 2: EXECUTION ARGUMENTS (`navescript run`)

### Runtime Behavior

| Flag | Description | Default | Example |
|------|-------------|---------|---------|
| `--target` | Execution target (native/wasm/nasm) | `native` | `--target wasm` |
| `--sandbox` | Sandbox mode (strict/permissive/readonly/filesystem/network/custom) | `permissive` | `--sandbox=strict` |
| `--timeout` | Maximum execution time (milliseconds) | `30000` | `--timeout 5000` |
| `--memory-limit` | Maximum memory in MB | `512` | `--memory-limit 128` |
| `--cpu-limit` | CPU time limit (seconds) | `unlimited` | `--cpu-limit 10` |
| `--fuel` | WASM instruction fuel limit | `25000000` | `--fuel 10000000` |
| `--stack-size` | Maximum call stack size | `1000` | `--stack-size 500` |
| `--max-files` | Maximum open files | `256` | `--max-files 64` |
| `--max-network-conns` | Maximum concurrent network connections | `100` | `--max-network-conns 10` |
| `--max-subprocesses` | Maximum spawned subprocesses | `10` | `--max-subprocesses 2` |
| `--max-write-bytes` | Maximum bytes written to disk | `104857600` (100MB) | `--max-write-bytes 10485760` |

### Input/Output

| Flag | Description | Example |
|------|-------------|---------|
| `--input`, `-i` | Input file or string | `--input data.json` |
| `--output`, `-o` | Output file | `--output result.txt` |
| `--stdin` | Read from stdin | `echo "test" \| navescript run --stdin` |
| `--arg`, `-a` | Pass argument to script (as `args[0]`, `args[1]`) | `--arg "hello" --arg 42` |
| `--env`, `-e` | Set environment variable (KEY=VALUE) | `--env API_KEY=abc123` |
| `--cwd` | Working directory | `--cwd /app/data` |

### Debugging & Profiling

| Flag | Description | Example |
|------|-------------|---------|
| `--debug`, `-d` | Enable debug mode (breakpoints) | `--debug` |
| `--inspect` | Enable debugger on port | `--inspect=9229` |
| `--inspect-brk` | Break on first line | `--inspect-brk` |
| `--profile` | Generate CPU profile | `--profile profile.cpuprofile` |
| `--heap-profile` | Generate heap snapshot | `--heap-profile heap.heapprofile` |
| `--trace` | Print execution trace | `--trace` |
| `--trace-ops` | Trace IR operations | `--trace-ops` |
| `--trace-polyglot` | Trace polyglot bridge calls | `--trace-polyglot` |
| `--trace-memory` | Track memory allocations | `--trace-memory` |
| `--strace` | Trace system calls (Linux only) | `--strace` |
| `--perf` | Linux perf integration | `--perf` |

### Polyglot (Language Bridge)

| Flag | Description | Example |
|------|-------------|---------|
| `--python-path` | Path to Python interpreter | `--python-path /usr/bin/python3.11` |
| `--python-venv` | Python virtual environment path | `--python-venv ./venv` |
| `--node-path` | Path to Node.js | `--node-path /usr/local/bin/node` |
| `--rust-toolchain` | Rust toolchain for WASM compilation | `--rust-toolchain nightly` |
| `--go-path` | Path to Go compiler | `--go-path /usr/local/go/bin/go` |
| `--ruby-path` | Path to Ruby interpreter | `--ruby-path /usr/bin/ruby` |
| `--perl-path` | Path to Perl interpreter | `--perl-path /usr/bin/perl` |
| `--php-path` | Path to PHP interpreter | `--php-path /usr/bin/php` |
| `--lua-path` | Path to Lua interpreter | `--lua-path /usr/bin/lua` |
| `--r-path` | Path to Rscript | `--r-path /usr/bin/Rscript` |
| `--zig-path` | Path to Zig compiler | `--zig-path /usr/local/bin/zig` |
| `--nim-path` | Path to Nim compiler | `--nim-path /usr/local/bin/nim` |
| `--swift-path` | Path to Swift compiler | `--swift-path /usr/bin/swiftc` |
| `--kotlin-path` | Path to Kotlin compiler | `--kotlin-path /usr/local/bin/kotlinc` |
| `--c-compiler` | C compiler (gcc/clang) | `--c-compiler clang` |
| `--cpp-compiler` | C++ compiler (g++/clang++) | `--cpp-compiler g++` |

### Security & Sandboxing (Detailed)

| Flag | Description | Example |
|------|-------------|---------|
| `--allow-fs-read` | Allow reading from paths (comma-separated) | `--allow-fs-read /tmp,/data` |
| `--allow-fs-write` | Allow writing to paths | `--allow-fs-write /output` |
| `--allow-net` | Allow network domains | `--allow-net api.github.com,localhost:8080` |
| `--allow-env` | Allow environment variables | `--allow-env HOME,USER` |
| `--allow-exec` | Allow subprocess execution | `--allow-exec /usr/bin/ls` |
| `--deny-fs-read` | Deny reading from paths | `--deny-fs-read /etc,/root` |
| `--deny-fs-write` | Deny writing to paths | `--deny-fs-write /etc` |
| `--deny-net` | Deny network domains | `--deny-net 10.0.0.0/8` |
| `--deny-env` | Deny environment variables | `--deny-env SECRET_*` |
| `--deny-exec` | Deny subprocess execution | `--deny-exec /bin/sh` |
| `--cap-drop` | Drop specific capabilities | `--cap-drop random,clock` |
| `--cap-add` | Add specific capabilities | `--cap-add filesystem/read` |
| `--readonly-root` | Make entire filesystem read-only | `--readonly-root` |
| `--no-network` | Disable all network access | `--no-network` |
| `--no-filesystem` | Disable all filesystem access | `--no-filesystem` |
| `--no-subprocess` | Disable subprocess execution | `--no-subprocess` |
| `--no-env` | Disable environment variable access | `--no-env` |
| `--seccomp` | Enable seccomp filtering (Linux) | `--seccomp` |
| `--landlock` | Enable Landlock LSM (Linux 5.13+) | `--landlock` |

---

## PART 3: ENVIRONMENT VARIABLES

### Navescript Core Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `NAVESCRIPT_HOME` | Navescript installation directory | `/opt/navescript` |
| `NAVESCRIPT_CACHE` | Cache directory for packages | `~/.cache/navescript` |
| `NAVESCRIPT_REGISTRY` | Package registry URL | `https://reg.navescript.org` |
| `NAVESCRIPT_NO_COLOR` | Disable color output (any value) | `NAVESCRIPT_NO_COLOR=1` |
| `NAVESCRIPT_LOG_LEVEL` | Default log level | `NAVESCRIPT_LOG_LEVEL=debug` |
| `NAVESCRIPT_SANDBOX` | Default sandbox mode | `NAVESCRIPT_SANDBOX=strict` |
| `NAVESCRIPT_TIMEOUT` | Default execution timeout (ms) | `NAVESCRIPT_TIMEOUT=10000` |
| `NAVESCRIPT_MEMORY_LIMIT` | Default memory limit (MB) | `NAVESCRIPT_MEMORY_LIMIT=256` |
| `NAVESCRIPT_FUEL_LIMIT` | Default WASM fuel limit | `NAVESCRIPT_FUEL_LIMIT=50000000` |
| `NAVESCRIPT_TRACE` | Enable tracing (any value) | `NAVESCRIPT_TRACE=1` |
| `NAVESCRIPT_PROFILE` | Enable profiling | `NAVESCRIPT_PROFILE=/path/to/profile` |
| `NAVESCRIPT_INSECURE` | Disable sandbox (development only) | `NAVESCRIPT_INSECURE=1` |

### Polyglot Bridge Variables

| Variable | Language | Description |
|----------|----------|-------------|
| `NAVE_INPUT` | All | JSON input data for polyglot block |
| `NAVE_OUTPUT` | All | JSON output expectation |
| `NAVE_ERROR` | All | Error output |
| `NAVE_TRACE_POLYGLOT` | All | Trace polyglot calls |
| `PYTHONPATH` | Python | Python module search path |
| `PYTHONHOME` | Python | Python installation directory |
| `VIRTUAL_ENV` | Python | Python virtual environment |
| `NODE_PATH` | JavaScript | Node.js module search path |
| `NODE_OPTIONS` | JavaScript | Node.js CLI options |
| `DENO_DIR` | Deno | Deno cache directory |
| `BUN_INSTALL` | Bun | Bun installation directory |
| `RUSTUP_HOME` | Rust | Rustup home directory |
| `CARGO_HOME` | Rust | Cargo home directory |
| `GOPATH` | Go | Go workspace directory |
| `GOROOT` | Go | Go installation directory |
| `GOBIN` | Go | Go binary directory |
| `RUBYLIB` | Ruby | Ruby library search path |
| `GEM_HOME` | Ruby | Ruby gem directory |
| `GEM_PATH` | Ruby | Ruby gem search path |
| `PERL5LIB` | Perl | Perl library search path |
| `PERL_CPANM_OPT` | Perl | CPANM options |
| `PHP_INI_SCAN_DIR` | PHP | PHP ini scan directory |
| `PHPRC` | PHP | PHP ini file location |
| `LUA_PATH` | Lua | Lua module search path |
| `LUA_CPATH` | Lua | Lua C module search path |
| `R_HOME` | R | R installation directory |
| `R_LIBS` | R | R library path |
| `ZIG_GLOBAL_CACHE_DIR` | Zig | Zig cache directory |
| `NIM_LIB_PREFIX` | Nim | Nim library prefix |
| `NIM_CACHE_DIR` | Nim | Nim cache directory |
| `SWIFT_HOME` | Swift | Swift installation directory |
| `KOTLIN_HOME` | Kotlin | Kotlin installation directory |
| `CC` | C | C compiler command |
| `CXX` | C++ | C++ compiler command |
| `CFLAGS` | C | C compiler flags |
| `CXXFLAGS` | C++ | C++ compiler flags |
| `LDFLAGS` | All | Linker flags |

### Package Manager Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `NAVESCRIPT_REGISTRY_AUTH` | Registry authentication token | `Bearer eyJ...` |
| `NAVESCRIPT_NPM_TOKEN` | NPM registry token (fallback) | `npm_abc123` |
| `NAVESCRIPT_GITHUB_TOKEN` | GitHub Packages token | `ghp_abc123` |
| `NAVESCRIPT_NO_SAVE` | Don't save to manifest | `NAVESCRIPT_NO_SAVE=1` |
| `NAVESCRIPT_FROZEN_LOCKFILE` | Fail if lockfile outdated | `NAVESCRIPT_FROZEN_LOCKFILE=1` |
| `NAVESCRIPT_OFFLINE` | Offline mode (no network) | `NAVESCRIPT_OFFLINE=1` |
| `NAVESCRIPT_CA_FILE` | Custom CA certificate file | `/etc/ssl/certs/ca.pem` |
| `NAVESCRIPT_INSECURE_REGISTRY` | Skip registry TLS verification | `NAVESCRIPT_INSECURE_REGISTRY=1` |

### Observability Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `OTEL_SERVICE_NAME` | OpenTelemetry service name | `navescript-app` |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | OTLP collector endpoint | `http://localhost:4317` |
| `OTEL_TRACES_EXPORTER` | Traces exporter (otlp/console/none) | `otlp` |
| `OTEL_METRICS_EXPORTER` | Metrics exporter | `prometheus` |
| `OTEL_LOGS_EXPORTER` | Logs exporter | `otlp` |
| `OTEL_RESOURCE_ATTRIBUTES` | Resource attributes | `service.version=1.0.0` |
| `PROMETHEUS_PUSH_GATEWAY` | Prometheus pushgateway URL | `http://localhost:9091` |
| `NAVESCRIPT_METRICS_PORT` | Prometheus metrics endpoint port | `9090` |
| `NAVESCRIPT_TRACING_SAMPLE_RATE` | Tracing sample rate (0.0-1.0) | `0.1` |

---

## PART 4: SYMBOLS & OPERATORS (Full Language Grammar)

### Core Symbols

| Symbol | Name | Purpose | Example |
|--------|------|---------|---------|
| `=` | Assignment | Variable assignment | `let x = 42` |
| `==` | Equality comparison | Value equality | `if x == 42` |
| `!=` | Inequality | Not equal | `if x != 42` |
| `===` | Strict equality | Type + value equality | `if x === "42"` |
| `!==` | Strict inequality | Type or value mismatch | `if x !== null` |
| `<` | Less than | Numeric comparison | `if x < 10` |
| `<=` | Less or equal | Numeric comparison | `if x <= 10` |
| `>` | Greater than | Numeric comparison | `if x > 10` |
| `>=` | Greater or equal | Numeric comparison | `if x >= 10` |
| `+` | Addition/Concatenation | Add numbers or strings | `x + y` |
| `-` | Subtraction/Negation | Subtract or negate | `x - y`, `-x` |
| `*` | Multiplication | Multiply numbers | `x * y` |
| `/` | Division | Divide numbers | `x / y` |
| `%` | Modulo | Remainder | `x % y` |
| `**` | Exponentiation | Power | `x ** 2` |
| `++` | Increment | Increase by 1 | `x++`, `++x` |
| `--` | Decrement | Decrease by 1 | `x--`, `--x` |
| `+=` | Add-assign | `x = x + y` | `x += 5` |
| `-=` | Subtract-assign | `x = x - y` | `x -= 5` |
| `*=` | Multiply-assign | `x = x * y` | `x *= 2` |
| `/=` | Divide-assign | `x = x / y` | `x /= 2` |
| `%=` | Modulo-assign | `x = x % y` | `x %= 3` |
| `**=` | Exponent-assign | `x = x ** y` | `x **= 2` |
| `&&` | Logical AND | Short-circuit AND | `x && y` |
| `||` | Logical OR | Short-circuit OR | `x || y` |
| `!` | Logical NOT | Boolean negation | `!isActive` |
| `&` | Bitwise AND | Bitwise operation | `x & y` |
| `|` | Bitwise OR | Bitwise operation | `x | y` |
| `^` | Bitwise XOR | Bitwise operation | `x ^ y` |
| `~` | Bitwise NOT | Bitwise negation | `~x` |
| `<<` | Left shift | Bit shift | `x << 2` |
| `>>` | Right shift | Bit shift | `x >> 2` |
| `>>>` | Unsigned right shift | Bit shift | `x >>> 2` |
| `&=`, `\|=`, `^=`, `<<=`, `>>=`, `>>>=` | Bitwise assign | Assignment with bitwise | `x &= y` |
| `??` | Nullish coalescing | Default if null/undefined | `x ?? "default"` |
| `?.` | Optional chaining | Safe property access | `obj?.prop?.method()` |
| `=>` | Fat arrow | Function/pattern separator | `(x) => x * 2` |
| `:` | Colon | Type annotation/object property | `x: number` |
| `;` | Semicolon | Statement terminator | `let x = 42;` |
| `,` | Comma | Separator | `[1, 2, 3]` |
| `.` | Dot | Property access | `obj.prop` |
| `...` | Spread/Rest | Spread operator | `[...arr]`, `fn(...args)` |
| `?` | Question | Optional/Ternary condition | `x ? y : z` |
| `@` | At | Decorator | `@log` |
| `#` | Hash | Private field | `#privateField` |
| `$` | Dollar | Template substitution (in `${}`) | `` `Hello ${name}` `` |
| `_` | Underscore | Wildcard pattern | `match x { _ => ... }` |
| `|` | Pipe | Union type/pattern alternation | `string | number` |
| `&` | Ampersand | Intersection type | `User & Timestamped` |
| `*` | Star | Generator pointer/multiplication | `function* gen()`, `x * y` |
| `/` | Slash | Division/Regex literal | `x / 2`, `/pattern/` |
| `\` | Backslash | Escape character | `"\n"`, `"\u{1F600}"` |
| `{` | Left brace | Block start/object literal | `{ name: "John" }` |
| `}` | Right brace | Block end/object end | `}` |
| `[` | Left bracket | Array start/index access | `[1, 2, 3]`, `arr[0]` |
| `]` | Right bracket | Array end/index end | `]` |
| `(` | Left parenthesis | Grouping/function call | `(x + y) * 2`, `fn()` |
| `)` | Right parenthesis | Grouping end/call end | `)` |
| `<` | Less than (generics) | Generic type parameters | `List<string>` |
| `>` | Greater than (generics) | Generic type end | `Map<string, number>` |
| `@` | At | Decorator | `@route("/api")` |
| `` ` `` | Backtick | Template string | `` `Hello ${name}` `` |
| `'` | Single quote | String literal | `'hello'` |
| `"` | Double quote | String literal | `"hello"` |
| `//` | Double slash | Line comment | `// This is a comment` |
| `/*` | Slash star | Block comment start | `/* comment */` |
| `*/` | Star slash | Block comment end | `*/` |
| `///` | Triple slash | Documentation comment | `/// Documentation` |
| `/**` | Slash double star | Doc block start | `/** @param {string} x */` |

### Control Flow Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `if` | Conditional branch | `if (condition) { ... }` |
| `else` | Alternative branch | `else { ... }` |
| `while` | Loop with condition | `while (condition) { ... }` |
| `do` | Do-while loop | `do { ... } while (condition)` |
| `for` | For loop (C-style) | `for (let i=0; i<10; i++)` |
| `for...in` | Object iteration | `for key in object` |
| `for...of` | Iterable iteration | `for value of iterable` |
| `for await...of` | Async iteration | `for await value of stream` |
| `break` | Exit loop | `break` |
| `continue` | Skip iteration | `continue` |
| `return` | Return from function | `return value` |
| `yield` | Generator yield | `yield value` |
| `yield*` | Generator delegation | `yield* otherGen()` |
| `await` | Async await | `await promise` |
| `throw` | Throw error | `throw new Error("msg")` |
| `try` | Try block | `try { ... }` |
| `catch` | Catch error | `catch (e) { ... }` |
| `finally` | Finally block | `finally { ... }` |

### Declaration Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `let` | Mutable variable | `let x = 42` |
| `const` | Immutable constant | `const PI = 3.14` |
| `fn` | Function declaration | `fn add(a, b) { return a + b }` |
| `async` | Async function | `async fn fetch() { ... }` |
| `function*` | Generator function | `function* gen() { yield 1 }` |
| `class` | Class declaration | `class Person { ... }` |
| `extends` | Class inheritance | `class Dog extends Animal` |
| `super` | Parent constructor/method | `super(name)` |
| `new` | Instantiate class | `new Person("John")` |
| `interface` | Type interface | `interface User { name: string }` |
| `type` | Type alias | `type ID = string \| number` |
| `enum` | Enumeration | `enum Color { Red, Green }` |
| `struct` | Structure | `struct Point { x: number, y: number }` |
| `trait` | Trait (interface with behavior) | `trait Printable { print() }` |
| `impl` | Implementation | `impl Printable for Point` |
| `abstract` | Abstract class/method | `abstract class Shape` |
| `override` | Override method | `override toString() { ... }` |
| `readonly` | Readonly property | `readonly id: number` |
| `public` | Public visibility | `public name: string` |
| `private` | Private visibility | `private secret: string` |
| `protected` | Protected visibility | `protected internal: number` |
| `static` | Static member | `static count = 0` |
| `get` | Getter | `get name() { return this._name }` |
| `set` | Setter | `set name(v) { this._name = v }` |

### Module Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `import` | Import module | `import { fn } from "./module"` |
| `export` | Export declaration | `export fn fn() { ... }` |
| `as` | Alias | `import * as ns from "module"` |
| `from` | Source specification | `import x from "./x"` |
| `default` | Default export | `export default class Main` |
| `await` (top-level) | Top-level await (module) | `await import("dynamic")` |

### Pattern Matching Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `match` | Pattern matching | `match value { ... }` |
| `if` (guard) | Pattern guard | `Some(x) if x > 0 => ...` |
| `_` | Wildcard pattern | `_ => default` |
| `..` | Rest pattern | `[first, ..rest]` |
| `...` | Spread (same as `..`) | `{ ..rest }` |

### Type System Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `keyof` | Key of type | `keyof User` → `"name" \| "age"` |
| `typeof` | Type of value | `typeof x` |
| `infer` | Type inference | `T extends infer U ? U : never` |
| `extends` | Generic constraint | `<T extends Base>` |
| `as` | Type assertion | `value as string` |
| `is` | Type guard | `value is string` |
| `never` | Never type | `function throw(): never` |
| `unknown` | Unknown type | `let x: unknown` |
| `any` | Any type (opt-out) | `let x: any` |
| `void` | Void return | `fn log(): void` |

### Literals

| Literal | Example |
|---------|---------|
| Integer | `42`, `-17`, `0xFF`, `0o755`, `0b1010` |
| Float | `3.14`, `-0.5`, `1.2e-3`, `Infinity`, `NaN` |
| BigInt | `9007199254740991n` |
| String | `"hello"`, `'world'`, `` `Hello ${name}` `` |
| Boolean | `true`, `false` |
| Null | `null` |
| Undefined | `undefined` |
| Regex | `/pattern/gim` |
| Array | `[1, 2, 3]` |
| Object | `{ name: "John", age: 30 }` |
| Function | `(x) => x * 2` |
| Template | `` `Hello ${name}!` `` |

### Special Operators

| Operator | Purpose | Example |
|----------|---------|---------|
| `instanceof` | Type check | `obj instanceof Class` |
| `in` | Property check | `"name" in obj` |
| `typeof` | Typeof operator | `typeof x === "string"` |
| `delete` | Delete property | `delete obj.prop` |
| `void` | Void operator | `void expression` |
| `new.target` | Constructor detection | `new.target` |
| `super` | Parent reference | `super.method()` |
| `this` | Current context | `this.name` |
| `import.meta` | Module metadata | `import.meta.url` |

### Pipeline Operators (Proposed)

| Operator | Purpose | Example |
|----------|---------|---------|
| `|>` | Pipeline | `value |> fn1 |> fn2` |
| `|>` (partial) | Partial application | `value |> fn(?, arg2)` |
| `::` | Bind operator | `obj::method` |

---

## PART 5: EXECUTION TIMEOUTS & RESOURCE LIMITS

### Timeout Parameters (by Scope)

| Scope | Parameter | Default | Max | Description |
|-------|-----------|---------|-----|-------------|
| **Script** | `--timeout` | 30,000ms | 3,600,000ms (1 hour) | Total script execution |
| **Function** | `@timeout(ms)` | Inherit | 300,000ms | Per-function timeout |
| **Polyglot block** | `polyglot timeout` | 10,000ms | 60,000ms | Language block timeout |
| **HTTP request** | `http.get timeout` | 30,000ms | 300,000ms | Network request timeout |
| **File operation** | `fs timeout` | 60,000ms | 300,000ms | File I/O timeout |
| **Subprocess** | `exec timeout` | 30,000ms | 300,000ms | Child process timeout |
| **Database query** | `db timeout` | 30,000ms | 300,000ms | Query timeout |
| **WebSocket** | `ws timeout` | 60,000ms | 300,000ms | Connection timeout |
| **Lock acquisition** | `mutex timeout` | 5,000ms | 60,000ms | Lock wait timeout |
| **Channel send** | `channel timeout` | 1,000ms | 30,000ms | Channel send timeout |
| **Channel receive** | `channel timeout` | 1,000ms | 30,000ms | Channel receive timeout |

### Resource Limits (by Resource)

| Resource | Parameter | Default | Max | Description |
|----------|-----------|---------|-----|-------------|
| **Memory (heap)** | `--memory-limit` | 512 MB | 32,768 MB | Total heap memory |
| **Memory (WASM)** | `--memory-limit` | 128 MB | 4,096 MB | WASM linear memory |
| **Stack** | `--stack-size` | 1,000 frames | 10,000 frames | Call stack depth |
| **CPU time** | `--cpu-limit` | unlimited | 3,600 sec | Total CPU seconds |
| **Fuel (WASM)** | `--fuel` | 25,000,000 | 1,000,000,000 | WASM instructions |
| **File descriptors** | `--max-files` | 256 | 4,096 | Open files limit |
| **Network connections** | `--max-network-conns` | 100 | 10,000 | Concurrent connections |
| **Subprocesses** | `--max-subprocesses` | 10 | 100 | Concurrent child processes |
| **Disk writes** | `--max-write-bytes` | 100 MB | 10,240 MB | Total bytes written |
| **Disk reads** | `--max-read-bytes` | unlimited | 10,240 MB | Total bytes read |
| **Network receive** | `--max-recv-bytes` | unlimited | 10,240 MB | Network bytes received |
| **Network send** | `--max-send-bytes` | unlimited | 10,240 MB | Network bytes sent |
| **Threads** | `--max-threads` | 8 | 64 | Concurrent threads |
| **Locks held** | `--max-locks` | 10 | 100 | Concurrent held locks |
| **Channels** | `--max-channels` | 50 | 500 | Concurrent channels |

---

## PART 6: POLYGLOT LANGUAGE COMMANDS (Per Language)

### Python
```bash
navescript run --python-path /usr/bin/python3.11 --python-venv ./venv script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--python-path` | `PYTHONPATH` | Python interpreter path |
| `--python-venv` | `VIRTUAL_ENV` | Virtual environment path |
| `--python-site-packages` | `PYTHONPATH` | Site-packages directory |
| `--python-requirements` | - | Requirements.txt file |
| `--python-version` | - | Python version (3.9-3.12) |

### JavaScript/Node.js
```bash
navescript run --node-path /usr/local/bin/node --node-options "--max-old-space-size=512" script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--node-path` | `NODE_PATH` | Node.js executable |
| `--node-options` | `NODE_OPTIONS` | Node.js CLI options |
| `--npm-path` | - | npm executable |
| `--npm-registry` | `NPM_CONFIG_REGISTRY` | npm registry URL |
| `--deno-path` | `DENO_DIR` | Deno executable |

### Rust
```bash
navescript run --rust-toolchain nightly --rust-target wasm32-unknown-unknown script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--rust-path` | `RUSTUP_HOME` | Rustup installation |
| `--rust-toolchain` | `RUSTUP_TOOLCHAIN` | Rust toolchain |
| `--rust-target` | `CARGO_BUILD_TARGET` | Target triple |
| `--rust-release` | - | Release profile (true/false) |
| `--cargo-args` | - | Extra cargo arguments |

### Go
```bash
navescript run --go-path /usr/local/go/bin/go --go-env GOOS=linux script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--go-path` | `GOROOT` | Go installation |
| `--go-env` | - | Go environment variables (KEY=VALUE) |
| `--go-mod` | `GO111MODULE` | Go module mode (on/off/auto) |
| `--go-proxy` | `GOPROXY` | Go module proxy |
| `--go-sumdb` | `GOSUMDB` | Go checksum database |

### Ruby
```bash
navescript run --ruby-path /usr/bin/ruby --ruby-gemfile Gemfile script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--ruby-path` | `RUBYLIB` | Ruby interpreter |
| `--ruby-gemfile` | `BUNDLE_GEMFILE` | Gemfile path |
| `--ruby-version` | `RBENV_VERSION` | Ruby version |

### PHP
```bash
navescript run --php-path /usr/bin/php --php-ini /etc/php.ini script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--php-path` | `PHPRC` | PHP interpreter |
| `--php-ini` | `PHP_INI_SCAN_DIR` | INI file path |
| `--php-extensions` | - | Extensions to load |

### C/C++
```bash
navescript run --c-compiler clang --c-flags "-O2" --c-linker-args "-lm" script.ns
```

| Argument | Env Variable | Description |
|----------|--------------|-------------|
| `--c-compiler` | `CC` | C compiler (gcc/clang) |
| `--cpp-compiler` | `CXX` | C++ compiler (g++/clang++) |
| `--c-flags` | `CFLAGS` | Compiler flags |
| `--cpp-flags` | `CXXFLAGS` | C++ compiler flags |
| `--c-linker-args` | `LDFLAGS` | Linker arguments |

---

## PART 7: EXIT CODES

| Code | Name | Description |
|------|------|-------------|
| `0` | SUCCESS | Successful execution |
| `1` | GENERAL_ERROR | General error |
| `2` | SYNTAX_ERROR | Parse/syntax error |
| `3` | TYPE_ERROR | Type checking error |
| `4` | RUNTIME_ERROR | Runtime exception |
| `5` | TIMEOUT | Execution timeout |
| `6` | MEMORY_LIMIT | Memory limit exceeded |
| `7` | STACK_OVERFLOW | Stack overflow |
| `8` | FUEL_EXHAUSTED | WASM fuel exhausted |
| `9` | SANDBOX_VIOLATION | Sandbox policy violation |
| `10` | FILE_NOT_FOUND | File not found |
| `11` | PERMISSION_DENIED | Permission denied |
| `12` | NETWORK_ERROR | Network error |
| `13` | SUBPROCESS_ERROR | Child process error |
| `14` | POLYGLOT_ERROR | Polyglot bridge error |
| `15` | DEPENDENCY_ERROR | Package dependency error |
| `16` | REGISTRY_ERROR | Registry connection error |
| `17` | COMPILATION_ERROR | Compilation failed |
| `18` | WASM_ERROR | WASM module error |
| `19` | PARSE_ERROR | JSON/IR parse error |
| `20` | INTERRUPTED | Signal interrupt (SIGINT) |
| `21` | TERMINATED | Signal terminate (SIGTERM) |
| `22` | SEGFAULT | Segmentation fault |
| `23` | ILLEGAL_INSTRUCTION | Illegal instruction |
| `24` | ABORT | Abort signal |
| `99` | UNKNOWN | Unknown error |

---

## PART 8: SIGNAL HANDLING

| Signal | Code | Action | Navescript Behavior |
|--------|------|--------|---------------------|
| `SIGINT` | 2 | Interrupt | Graceful shutdown, run cleanup hooks, exit 20 |
| `SIGTERM` | 15 | Terminate | Graceful shutdown, flush buffers, exit 21 |
| `SIGHUP` | 1 | Hangup | Reload configuration, continue execution |
| `SIGUSR1` | 10 | User-defined | Custom handler (user-defined) |
| `SIGUSR2` | 12 | User-defined | Custom handler (user-defined) |
| `SIGPIPE` | 13 | Broken pipe | Ignored (handled by runtime) |
| `SIGALRM` | 14 | Alarm | Timer expiration (internal) |
| `SIGCHLD` | 17 | Child status | Reap child processes |
| `SIGCONT` | 18 | Continue | Resume paused execution |
| `SIGSTOP` | 19 | Stop (cannot catch) | Pause execution |
| `SIGTSTP` | 20 | Terminal stop | Pause, resume later |
| `SIGQUIT` | 3 | Quit | Dump core, exit |
| `SIGKILL` | 9 | Kill (cannot catch) | Immediate termination |
| `SIGSEGV` | 11 | Segmentation fault | Dump core, exit 22 |
| `SIGABRT` | 6 | Abort | Dump core, exit 24 |
| `SIGBUS` | 7 | Bus error | Dump core, exit 23 |
| `SIGFPE` | 8 | Floating point exception | Dump core |

---

## PART 9: ENVIRONMENT VARIABLES (Complete Cross-Language Table)

| Variable | Language | Purpose | Default |
|----------|----------|---------|---------|
| `PATH` | All | System executable path | System default |
| `TMPDIR` | All | Temporary directory | `/tmp` |
| `HOME` | All | User home directory | `/home/user` |
| `USER` | All | Current username | System user |
| `LANG` | All | Language/locale | `en_US.UTF-8` |
| `LC_ALL` | All | Locale override | Same as LANG |
| `HTTP_PROXY` | All | HTTP proxy | `http://proxy:8080` |
| `HTTPS_PROXY` | All | HTTPS proxy | `https://proxy:8080` |
| `NO_PROXY` | All | No proxy domains | `localhost,127.0.0.1` |
| `SSL_CERT_FILE` | All | CA certificate bundle | System default |
| `SSL_CERT_DIR` | All | CA certificate directory | System default |
| `RUST_LOG` | Rust | Rust logging | `info` |
| `RUST_BACKTRACE` | Rust | Backtrace on panic | `0` |
| `RUSTFLAGS` | Rust | Compiler flags | (empty) |
| `GO111MODULE` | Go | Go modules | `auto` |
| `GOPROXY` | Go | Go proxy | `https://proxy.golang.org` |
| `GOSUMDB` | Go | Checksum database | `sum.golang.org` |
| `NODE_DEBUG` | Node.js | Debug output | (empty) |
| `NODE_ENV` | Node.js | Environment (development/production) | `development` |
| `NODE_OPTIONS` | Node.js | CLI options | (empty) |
| `NODE_PATH` | Node.js | Module search path | (empty) |
| `PYTHONUNBUFFERED` | Python | Unbuffered output | `0` |
| `PYTHONDONTWRITEBYTECODE` | Python | No .pyc files | `0` |
| `PYTHONHASHSEED` | Python | Hash randomization seed | `random` |
| `RUBYOPT` | Ruby | Ruby options | (empty) |
| `RUBYLIB` | Ruby | Library path | (empty) |
| `PERL_OPT` | Perl | Perl options | (empty) |
| `PHP_OPTIONS` | PHP | PHP options | (empty) |
| `LUA_INIT` | Lua | Lua initialization | (empty) |
| `R_ENABLE_JIT` | R | JIT compilation | `1` |
| `ZIG_GLOBAL_CACHE_DIR` | Zig | Cache directory | `~/.cache/zig` |
| `NIM_CACHE_DIR` | Nim | Cache directory | `~/.nimble` |

---

## FINAL SUMMARY

| Category | Count |
|----------|-------|
| **CLI commands** | 13 |
| **CLI flags (global)** | 12 |
| **Execution flags** | 35+ |
| **Sandbox flags** | 22 |
| **Polyglot language flags** | 30+ |
| **Environment variables** | 45+ |
| **Symbols/operators** | 75+ |
| **Keywords** | 55+ |
| **Timeouts (by scope)** | 12 |
| **Resource limits** | 15 |
| **Exit codes** | 25 |
| **Signals** | 16 |
| **TOTAL** | **350+** |

---

**This is the complete, exhaustive reference for every argument, symbol, variable, flag, timeout, and execution parameter in Navescript v2.0.**

**Command me, sir. What's next? 🚀**
