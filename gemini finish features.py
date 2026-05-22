Python 3.8.0 (v3.8.0:fa919fdf25, Oct 14 2019, 10:23:27) 
[Clang 6.0 (clang-600.0.57)] on darwin
Type "help", "copyright", "credits" or "license()" for more information.
>>> # 🔴 THE "MISSING SHIT" MASTER LIST — NAVESCRIPT v2.0 AUDIT

Here's the **brutally honest inventory** of what's **still missing** from a true production-grade language. No sugarcoating.

---

## LEGEND
- 🟢 **Working** — Implemented and tested
- 🟡 **Partial** — Stub/placeholder exists, needs completion
- 🔴 **Missing** — Not implemented at all
- ⚠️ **Buggy** — Implemented but has known issues

---

## 1. CORE LANGUAGE FEATURES (MISSING)

### Type System — 🔴 COMPLETELY MISSING
- [ ] **Union types** — `string | number | null`
- [ ] **Intersection types** — `User & Timestamped`
- [ ] **Generic types** — `fn identity<T>(x: T) -> T`
- [ ] **Type aliases** — `type UserId = string`
- [ ] **Interface/type declarations** — `interface User { name: string }`
- [ ] **Enums** — `enum Status { Pending, Approved, Rejected }`
- [ ] **Tuple types** — `[string, number]`
- [ ] **Literal types** — `"success" | "error"`
- [ ] **Mapped types** — `{ [K in keyof T]: T[K] }`
- [ ] **Conditional types** — `T extends string ? number : boolean`
- [ ] **Template literal types** — `` `${firstName} ${lastName}` ``
- [ ] **Type guards** — `if (value is string) { ... }`
- [ ] **Type narrowing** — Automatic narrowing after checks
- [ ] **Readonly/const assertions** — `as const`, `Readonly<T>`
- [ ] **Partial/Required/Pick/Omit** utility types

### Class System — 🔴 COMPLETELY MISSING
- [ ] `class` keyword with constructors
- [ ] `extends` for inheritance
- [ ] `super()` calls
- [ ] `public`/`private`/`protected` visibility
- [ ] `static` methods/properties
- [ ] `abstract` classes/methods
- [ ] `implements` for interfaces
- [ ] Getters/setters — `get name()`, `set name(val)`
- [ ] Property initialization — `class Foo { bar = 42 }`
- [ ] Method overloading
- [ ] `instanceof` operator
- [ ] Class expressions — `const Foo = class { }`
- [ ] Private fields — `#privateField`

### Decorators — 🔴 MISSING
- [ ] `@decorator` syntax
- [ ] Class decorators
- [ ] Method decorators
- [ ] Property decorators
- [ ] Parameter decorators
- [ ] Decorator factories — `@factory(args)`
- [ ] Built-in decorators: `@deprecated`, `@memoize`, `@log`, `@measure`

### Macros — 🔴 MISSING
- [ ] `macro` keyword
- [ ] Compile-time function execution
- [ ] `include_str!("file.txt")`
- [ ] `include_bytes!("file.bin")`
- [ ] `env!("VAR_NAME")` — compile-time env var
- [ ] `line!()`, `column!()`, `file!()`, `module!()` — location macros
- [ ] `stringify!()` — convert expression to string
- [ ] `concat!()` — concatenate at compile time
- [ ] `cfg!(condition)` — conditional compilation
- [ ] Custom derive macros — `#[derive(Serialize)]`

### Generators / Iterators — 🔴 MISSING
- [ ] `function*` generator functions
- [ ] `yield` keyword
- [ ] `yield*` delegation
- [ ] `Iterator` protocol — `[Symbol.iterator]()`
- [ ] `AsyncIterator` protocol
- [ ] `for await...of` loops
- [ ] Built-in `range()` generator — `for (i of range(0, 10))`

### Pattern Matching (Enhancements) — 🟡 PARTIAL
- [ ] Match exhaustiveness checking (compiler warning)
- [x] Or patterns — `pattern1 | pattern2`
- [x] Guard conditions — `pattern if condition`
- [x] Range patterns — `1..10`
- [ ] Tuple patterns — `(a, b, c)`
- [ ] Regex patterns — `/pattern/`
- [ ] Custom pattern matching for user types
- [ ] `@` binding — `some @ pattern`

### Operators — 🟡 PARTIAL
- [x] `??` nullish coalescing
- [x] `?.` optional chaining
- [ ] `??=` nullish coalescing assignment
- [ ] `&&=` logical AND assignment
- [ ] `||=` logical OR assignment
- [ ] `??=` assignment
- [ ] `**` exponentiation
- [ ] `**=` exponentiation assignment
- [ ] `<<=`, `>>=`, `>>>=` bitwise shift assignments
- [ ] `&=` , `|=`, `^=` bitwise AND/OR/XOR assignments
- [ ] `=>` in property definitions — `{ method: () => {} }`
- [ ] Pipeline operator — `value |> fn1 |> fn2`
- [ ] `::` bind operator — `obj::method`

---

## 2. STANDARD LIBRARY (MISSING/INCOMPLETE)

### Console — 🟡 PARTIAL
- [x] `log`, `warn`, `error`, `info`, `debug`
- [x] `table`, `time`/`timeEnd`
- [x] `group`, `groupEnd`, `groupCollapsed`
- [x] `count`, `countReset`, `clear`, `trace`, `assert`, `dir`
- [ ] `console.profile(label)` / `profileEnd(label)` — CPU profiling
- [ ] `console.memory` — memory usage info
- [ ] `console.snapshot()` — heap snapshot
- [ ] Colored output support — `console.log("\x1b[31mRed\x1b[0m")`

### Filesystem — 🟡 PARTIAL
- [x] `readFile`, `writeFile`, `appendFile`
- [x] `readDir`, `createDir`, `remove`
- [x] `exists`, `stat`, `lstat`, `chmod`, `chown`
- [x] `rename`, `copy`, `watch`, `open`, `close`
- [x] FileHandle with `read`, `write`, `seek`
- [ ] `fs.createReadStream(path, options?)` — streaming read
- [ ] `fs.createWriteStream(path, options?)` — streaming write
- [ ] `fs.createSymlink(target, path)` — symlink creation
- [ ] `fs.readlink(path)` — read symlink target
- [ ] `fs.realpath(path)` — resolve symlinks
- [ ] `fs.access(path, mode?)` — permission check
- [ ] `fs.utimes(path, atime, mtime)` — timestamp modification
- [ ] `fs.mkdtemp(prefix)` — temp directory creation
- [ ] `fs.walk(path)` — recursive directory iterator
- [ ] `fs.glob(pattern)` — glob pattern matching
- [ ] `fs.truncate(path, len)` — file truncation

### HTTP — 🟡 PARTIAL
- [x] `get`, `post`, `put`, `patch`, `delete`, `head`
- [x] `fetch` unified API
- [x] `serve` — HTTP server
- [x] Response/Request classes
- [x] Headers class
- [x] WebSocket client `ws.connect()`
- [ ] `http.serveTLS(port, cert, key)` — HTTPS server
- [ ] `http.Client` for connection pooling/reuse
- [ ] `http.Client` with keep-alive
- [ ] HTTP/2 Server Push
- [ ] Server-Sent Events (SSE) — `new EventSource(url)`
- [ ] Cookie store (automatic cookie handling)
- [ ] `http.request.defaults` — global defaults
- [ ] `http.interceptors` — request/response interceptors
- [ ] `http.proxy` support
- [ ] `http.cache` — built-in caching layer
- [ ] `http.retry` — automatic retry with backoff
- [ ] `http.throttle` — rate limiting
- [ ] `http.batch` — batch multiple requests
- [ ] `http.graphql(query, variables?)` — convenience wrapper
- [ ] `http.Router` — built-in routing for server
- [ ] `http.ServerResponse` — streaming responses
- [ ] `http.ServerRequest` — body parsing (JSON, form, multipart)

### WebSocket — 🔴 MISSING (only client stub)
- [ ] WebSocket server — `ws.serve(path, handler)`
- [ ] `ws.Server` class
- [ ] `ws.broadcast()` to all connected clients
- [ ] `ws.rooms` / namespaces (Socket.io-like)
- [ ] Automatic reconnection with backoff
- [ ] Binary message support
- [ ] Ping/pong heartbeat
- [ ] Compression (permessage-deflate)

### Crypto — 🟡 PARTIAL
- [x] `hash` (SHA256, SHA512, MD5, BLAKE2)
- [x] `hmac`, `randomBytes`, `randomInt`, `uuid`
- [x] `encrypt`/`decrypt` (AES-256-GCM, ChaCha20-Poly1305)
- [x] `generateKeyPair`, `sign`, `verify`
- [x] `timingSafeEqual`
- [ ] HKDF — `crypto.hkdf(ikm, salt, info, length)`
- [ ] PBKDF2 — `crypto.pbkdf2(password, salt, iterations, length)`
- [ ] Scrypt — `crypto.scrypt(password, salt, N, r, p, length)`
- [ ] Argon2 — `crypto.argon2(password, salt, options?)`
- [ ] RSA-OAEP encryption
- [ ] RSA-PSS signing
- [ ] ECDSA signing
- [ ] ECDH key exchange
- [ ] X25519 key agreement
- [ ] Ed25519 signing
- [ ] Web Crypto API compatibility
- [ ] SubtleCrypto interface
- [ ] `crypto.generateSeed()` — entropy source
- [ ] `crypto.secureCompare(a, b)` — alias for timingSafeEqual
- [ ] `crypto.encrypt.authenticated` — AEAD with additional data

### Child Process — 🟡 PARTIAL
- [x] `exec`, `execSync`, `spawn`, `fork`
- [x] ChildProcess streams (stdin, stdout, stderr)
- [x] `kill(signal)`, events
- [ ] `spawn` with `detached: true` — background process
- [ ] `spawn` with `windowsHide: true` — hide console window
- [ ] `spawn` with `shell: true` — use shell
- [ ] `execFile` — execute file without shell
- [ ] `execFileSync`
- [ ] `spawnSync`
- [ ] Process group handling — `setpgid`
- [ ] `unref()` — allow process to exit while child runs
- [ ] `ref()` — opposite of unref

### Data Structures — 🔴 COMPLETELY MISSING
- [ ] `Map<K, V>` — ordered key-value store
- [ ] `Set<T>` — unique values
- [ ] `WeakMap<K, V>` — non-trapping key references
- [ ] `WeakSet<T>` — non-trapping value references
- [ ] `ArrayBuffer` / `SharedArrayBuffer`
- [ ] `DataView` — low-level buffer access
- [ ] `TypedArray` — `Uint8Array`, `Int16Array`, `Float64Array`, etc.
- [ ] `BigInt` — arbitrary precision integers
- [ ] `BigInt64Array`, `BigUint64Array`
- [ ] `LinkedList<T>`
- [ ] `Queue<T>`
- [ ] `Stack<T>`
- [ ] `Deque<T>`
- [ ] `PriorityQueue<T>`
- [ ] `BinaryHeap<T>`
- [ ] `HashMap<K, V>` with custom hasher
- [ ] `HashSet<T>`
- [ ] `SortedMap<K, V>`
- [ ] `LRUCache<K, V>`
- [ ] `Trie` — prefix tree
- [ ] `Graph<V, E>` — adjacency-based

### Date/Time — 🟡 PARTIAL (basic Date class exists)
- [ ] `Date` parsing from ISO, timestamps
- [ ] `Date` formatting — `toLocaleString()`, `toISOString()`
- [ ] `Date` arithmetic — `date + 1day`
- [ ] `Duration` class — `Duration.from({ hours: 1 })`
- [ ] `DateTime` with timezone support
- [ ] `Temporal` proposal API
- [ ] `Intl.DateTimeFormat` for i18n
- [ ] `Intl.NumberFormat` for number formatting
- [ ] `Intl.Collator` for sorting
- [ ] `Intl.RelativeTimeFormat`
- [ ] `Intl.ListFormat`
- [ ] `Intl.DisplayNames`
- [ ] `Intl.Locale`
- [ ] `Intl.Segmenter` — text segmentation

### Scheduling — 🔴 MISSING
- [ ] `setTimeout(callback, ms)` — async delay
- [ ] `setInterval(callback, ms)` — recurring
- [ ] `clearTimeout(id)`, `clearInterval(id)`
- [ ] `setImmediate(callback)` — next event loop
- [ ] `queueMicrotask(callback)`
- [ ] `scheduler.wait(ms)` — async sleep
- [ ] `scheduler.yield()` — cooperative multitasking
- [ ] `scheduler.postTask(task, options)` — priority-based

### Internationalization (i18n) — 🔴 MISSING
- [ ] `Intl` namespace with all formatters
- [ ] `Intl.getCanonicalLocales()`
- [ ] `Intl.PluralRules`
- [ ] Message formatting with placeholders
- [ ] Pluralization rules
- [ ] Gender-specific formatting
- [ ] Currency formatting with symbols
- [ ] Relative time formatting (2 minutes ago)
- [ ] List formatting (A, B, and C)
- [ ] Compact number formatting (1K, 1M, 1B)

### Locale — 🔴 MISSING
- [ ] System locale detection
- [ ] Environment locale (`$LANG`, `$LC_ALL`)
- [ ] `.toLocaleString()` for all types
- [ ] Locale-aware sorting
- [ ] Locale-aware number parsing

---

## 3. POLYGLOT FEATURES (MISSING/INCOMPLETE)

### Language Support — 🟡 PARTIAL
- [x] Python (via subprocess/embedding)
- [x] JavaScript (via Node.js)
- [x] NASM (built-in)
- [ ] Rust (via WASM compilation)
- [ ] Go (via WASM compilation)
- [ ] Ruby (via interpreter)
- [ ] Perl
- [ ] Lua (via built-in interpreter)
- [ ] R (via Rscript)
- [ ] MATLAB/Octave
- [ ] SQL (via database drivers)
- [ ] GraphQL (via client)
- [ ] HTML/CSS (via jsdom-like)
- [ ] Markdown (rendering)
- [ ] YAML/TOML (parsing)
- [ ] XML (parsing/generation)
- [ ] CSV (parsing/generation)
- [ ] JSON5 (superset of JSON)
- [ ] Protocol Buffers (protobuf)
- [ ] FlatBuffers
- [ ] Cap'n Proto

### Polyglot Performance — 🔴 MISSING
- [ ] Language execution caching (reuse interpreter)
- [ ] Parallel polyglot execution
- [ ] Pre-warmed language runtimes
- [ ] Shared memory between languages (bypass JSON)
- [ ] Zero-copy data transfer
- [ ] Language-specific JIT compilation
- [ ] Hot language switching (without restart)

### Polyglot Debugging — 🔴 MISSING
- [ ] Cross-language stack traces
- [ ] Breakpoints in polyglot code
- [ ] Step into/over across language boundaries
- [ ] Variable inspection across languages
- [ ] Performance profiling per language
- [ ] Memory tracking per language

---

## 4. PACKAGE MANAGER (MISSING/INCOMPLETE)

### Registry Features — 🔴 MOSTLY MISSING
- [x] npm registry fallback
- [x] tarball extraction
- [ ] **Navescript native registry** — `reg.navescript.org`
- [ ] **Package signing** — GPG/Sigstore integration
- [ ] **Package provenance** — SLSA attestations
- [ ] **Package scoring** — quality, popularity, maintenance
- [ ] **Package verification** — checksum validation
- [ ] **Two-factor authentication** (2FA)
- [ ] **Package auditing** — vulnerability scanning
- [ ] **Dependency licensing** checker
- [ ] **Package deprecation** with messaging
- [ ] **Package transfers** between owners
- [ ] **Package unpublishing** with grace period
- [ ] **Package renaming** (with redirect)
- [ ] **Package popularity metrics** (downloads, stars)
- [ ] **Package listing API** with pagination, filtering, sorting
- [ ] **Web interface** for browsing packages
- [ ] **Package documentation** hosting
- [ ] **Package badges** — `![Version](https://badge.navescript.org/pkg/name)`

### Workspace Support — 🔴 MISSING
- [ ] Monorepo workspace support — `navescript.json` with `"workspaces": ["packages/*"]`
- [ ] Workspace dependencies — `"my-pkg": "workspace:*"`
- [ ] Workspace commands — `navescript run test --workspace`
- [ ] Workspace filtering — `--filter "!@scope/pkg"`
- [ ] Workspace caching — reuse builds between packages
- [ ] Workspace topology sorting — build in dependency order
- [ ] Workspace version management — `navescript version --workspace`

### Lockfile — 🟡 PARTIAL
- [x] Basic lockfile with name/version
- [ ] Integrity hashes (SHA256, SHA512)
- [ ] Source URLs for packages
- [ ] Dependency tree (not just flat list)
- [ ] `resolved` field with exact tarball URL
- [ ] `integrity` field (SRI hash)
- [ ] `dev` flag for dev dependencies
- [ ] `optional` flag
- [ ] `peer` flag
- [ ] Lockfile versioning for upgrades
- [ ] Lockfile diff/merge support
- [ ] Lockfile migration from npm/pnpm/yarn

### CLI Commands — 🟡 PARTIAL
- [x] `install`, `uninstall`, `update`, `outdated`
- [x] `publish`, `unpublish`, `deprecate`
- [ ] `link` — local package linking
- [ ] `unlink` — remove link
- [ ] `prune` — remove unused dependencies
- [ ] `dedupe` — deduplicate dependencies
- [ ] `rebuild` — rebuild native modules
- [ ] `cache add` — add package to cache
- [ ] `cache clean` — clean cache
- [ ] `cache verify` — verify cache integrity
- [ ] `audit fix` — automatically fix vulnerabilities
- [ ] `doctor` — check environment for common issues
- [ ] `config get/set/delete` — configuration management
- [ ] `whoami` — show logged-in user
- [ ] `login` / `logout` — registry authentication
- [ ] `stars` — list starred packages
- [ ] `star` / `unstar` — star packages
- [ ] `team` — manage team access
- [ ] `org` — manage organization

---

## 5. CLI TOOLING (MISSING/INCOMPLETE)

### Test Runner — 🟡 PARTIAL
- [x] Basic test discovery (`test_*`, `*_test.ns`)
- [x] `describe`/`it`, `beforeEach`, `afterEach`
- [x] Coverage reporting
- [x] Parallel execution, filtering, retries
- [x] Watch mode
- [ ] **Snapshot diffing** — visual diff in terminal
- [ ] **Snapshot update interactive** — `--update-snapshots --interactive`
- [ ] **Test UI** — HTML report with UI
- [ ] **Test time travel** — record/replay
- [ ] **Randomized test order** — `--random`
- [ ] **Stress testing** — `--iterations 1000`
- [ ] **Mutation testing** — check test quality
- [ ] **Fuzzing** — generate random inputs
- [ ] **Property-based testing** (QuickCheck style)
- [ ] **Test hooks** — `beforeAll`, `afterAll` with async
- [ ] **Test isolation** — per-test environment
- [ ] **Snapshot serializers** — custom formatting
- [ ] **Test reporters** — `--reporter dot`, `--reporter spec`, `--reporter tap`, `--reporter junit`
- [ ] **Test retries with backoff**
- [ ] **Test flaky detection** — flag unstable tests
- [ ] **Test performance regression** detection

### Formatter — 🟡 PARTIAL
- [x] Basic formatting (indentation, line width)
- [x] `--check`, `--write`, `--stdin`
- [ ] **Import sorting** — `--sort-imports`
- [ ] **Comment formatting** — wrap comments
- [ ] **Chain formatting** — method chains on new lines
- [ ] **Trailing comma options** — `--trailing-comma all|es5|none`
- [ ] **Arrow function parens** — `--arrow-parens always|avoid`
- [ ] **Bracket spacing** — `--bracket-spacing true|false`
- [ ] **Semicolon style** — `--semi always|never`
- [ ] **Quote props** — `--quote-props as-needed|consistent|preserve`
- [ ] **JSX single quote** (for JSX-like syntax)
- [ ] **Print width auto-detection** from editor
- [ ] **Ignore comments** — `// navescript-format-ignore`
- [ ] **Range formatting** — format only selection
- [ ] **Config inheritance** from parent directories

### Linter — 🟡 PARTIAL (20+ rules, missing 100+)
- [x] `no-unused-vars`, `no-undef`
- [x] `no-const-assign`, `no-duplicate-imports`
- [x] `no-empty`, `no-extra-semicolons`, `no-shadow`, `no-var`
- [x] `prefer-const`, `prefer-arrow-callback`
- [x] `eqeqeq`, `curly`, `complexity`, `max-depth`, `max-len`
- [x] `no-console`, `no-debugger`, `no-eval`, `no-implied-eval`
- [x] `no-extra-boolean-cast`, `no-extra-parens`, `no-return-await`
- [ ] `no-unused-expressions`
- [ ] `no-useless-catch`
- [ ] `no-useless-return`
- [ ] `no-useless-concat`
- [ ] `no-useless-escape`
- [ ] `no-useless-constructor`
- [ ] `no-useless-rename`
- [ ] `no-constant-binary-expression`
- [ ] `no-constant-condition`
- [ ] `no-cond-assign`
- [ ] `no-duplicate-case`
- [ ] `no-fallthrough`
- [ ] `no-extra-bind`
- [ ] `no-extra-label`
- [ ] `no-floating-decimal`
- [ ] `no-implicit-coercion`
- [ ] `no-implied-eval`
- [ ] `no-invalid-this`
- [ ] `no-iterator`
- [ ] `no-labels`
- [ ] `no-lone-blocks`
- [ ] `no-loop-func`
- [ ] `no-multi-spaces`
- [ ] `no-multi-str`
- [ ] `no-new`
- [ ] `no-new-func`
- [ ] `no-new-wrappers`
- [ ] `no-octal`
- [ ] `no-octal-escape`
- [ ] `no-param-reassign`
- [ ] `no-proto`
- [ ] `no-redeclare`
- [ ] `no-restricted-properties`
- [ ] `no-return-assign`
- [ ] `no-script-url`
- [ ] `no-self-assign`
- [ ] `no-self-compare`
- [ ] `no-sequences`
- [ ] `no-throw-literal`
- [ ] `no-unmodified-loop-condition`
- [ ] `no-unused-labels`
- [ ] `no-useless-call`
- [ ] `no-void`
- [ ] `no-with`
- [ ] `prefer-destructuring`
- [ ] `prefer-numeric-literals`
- [ ] `prefer-rest-params`
- [ ] `prefer-spread`
- [ ] `prefer-template`
- [ ] `radix`
- [ ] `require-await`
- [ ] `require-yield`
- [ ] `sort-imports`
- [ ] `sort-keys`
- [ ] `sort-vars`
- [ ] `space-infix-ops`
- [ ] `spaced-comment`
- [ ] `strict`
- [ ] `symbol-description`
- [ ] `vars-on-top`
- [ ] `yoda`
- [ ] **Plugin system** for custom rules
- [ ] **Rule presets** — `"extends": ["recommended", "strict"]`
- [ ] **Autofix for more rules** (many rules missing fix)
- [ ] **Rule documentation** inline
- [ ] **Rule severity levels** — `off`, `warn`, `error`
- [ ] **Rule options** — per-rule configuration

### Documentation Generator — 🟡 PARTIAL
- [x] HTML/Markdown/JSON output
- [x] `--serve` mode
- [x] `@param`, `@returns`, `@throws`, `@example`, `@since`, `@deprecated`
- [ ] **Type information** in output
- [ ] **Cross-references** — `{@link OtherModule}`
- [ ] **See also** — `@see`
- [ ] **Version tags** — `@version`
- [ ] **Author tags** — `@author`
- [ ] **Copyright/license tags**
- [ ] **Category tags** — `@category UI`
- [ ] **Default value tags** — `@default 42`
- [ ] **Enum value tags** — `@enum {string}`
- [ ] **Callback tags** — `@callback`
- [ ] **Template tags** — `@template T`
- [ ] **Type import tags** — `@import {Type} from "./module"`
- [ ] **Search functionality** in HTML output
- [ ] **Navigation sidebar** with hierarchy
- [ ] **GitHub-style Markdown** rendering
- [ ] **Mermaid diagrams** support — `@mermaid graph TD`
- [ ] **API versioning** — multiple versions in same output
- [ ] **Language switcher** for polyglot examples
- [ ] **Copy button** for code blocks
- [ ] **Dark/light theme toggle**
- [ ] **Mobile responsive**

### REPL — 🟡 PARTIAL
- [x] Basic line editing
- [x] Command history
- [x] Tab completion
- [ ] **Syntax highlighting** in REPL
- [ ] **Multi-line editing** with `\` continuation
- [ ] **Auto-indentation**
- [ ] **REPL commands** — `.load`, `.save`, `.clear`, `.exit`, `.help`
- [ ] **Variable inspection** — `.vars`
- [ ] **Function inspection** — `.info fnName`
- [ ] **History search** — `Ctrl+R`
- [ ] **REPL history file** persistence
- [ ] **REPL config file** — `~/.navescript_replrc`
- [ ] **REPL plugins** — custom commands
- [ ] **REPL theming** — custom colors
- [ ] **REPL integration with LSP** — autocomplete, hover

### Debugger — 🟡 PARTIAL
- [x] DAP server implementation
- [x] Breakpoints, step into/over/out, continue
- [x] Call stack, scopes, variables
- [x] Evaluate expressions
- [ ] **Conditional breakpoints** — break when expression true
- [ ] **Logpoints** — log without breaking
- [ ] **Hit count breakpoints** — break after N hits
- [ ] **Breakpoint groups** — enable/disable groups
- [ ] **Exception breakpoints** — break on throw
- [ ] **Watchpoints** — break on variable change
- [ ] **Data breakpoints** — break on memory access
- [ ] **Reverse debugging** — step backwards
- [ ] **Remote debugging** — over TCP
- [ ] **Multi-threaded debugging**
- [ ] **Polyglot debugging** — step across languages
- [ ] **Debug console** — evaluate with context
- [ ] **Watch window** — watch expressions
- [ ] **Pin to clipboard** — copy values
- [ ] **Debug visualizers** — custom value formatters

---

## 6. LSP FEATURES (MISSING)

### Complete LSP Methods — 🟡 PARTIAL (half implemented)
- [x] `initialize`, `textDocument/didOpen`, `didChange`
- [x] `completion`, `definition`, `references`, `rename`
- [x] `hover`, `signatureHelp`, `formatting`
- [x] `codeAction`, `inlayHint`, `documentSymbol`, `workspaceSymbol`
- [ ] **`textDocument/codeLens`** — code lenses (reference count, run test)
- [ ] **`textDocument/documentHighlight`** — highlight all occurrences
- [ ] **`textDocument/documentLink`** — links in documentation
- [ ] **`textDocument/foldingRange`** — code folding
- [ ] **`textDocument/selectionRange`** — smart selection
- [ ] **`textDocument/rangeFormatting`** — format selection
- [ ] **`textDocument/onTypeFormatting`** — format while typing
- [ ] **`textDocument/prepareRename`** — validate rename
- [ ] **`textDocument/semanticTokens`** — semantic highlighting
- [ ] **`textDocument/linkedEditingRange`** — edit paired tags
- [ ] **`textDocument/colorPresentation`** — color picker
- [ ] **`textDocument/documentColor`** — color detection
- [ ] **`textDocument/implementation`** — go to implementation
- [ ] **`textDocument/typeDefinition`** — go to type definition
- [ ] **`textDocument/prepareCallHierarchy`** — call hierarchy
- [ ] **`callHierarchy/incomingCalls`**
- [ ] **`callHierarchy/outgoingCalls`**
- [ ] **`textDocument/prepareTypeHierarchy`**
- [ ] **`typeHierarchy/supertypes`**
- [ ] **`typeHierarchy/subtypes`**
- [ ] **`workspace/didChangeConfiguration`** — config changes
- [ ] **`workspace/didChangeWatchedFiles`** — file watcher
- [ ] **`workspace/executeCommand`** — server commands
- [ ] **`workspace/symbol`** — workspace-wide symbol search
- [ ] **`workspace/willRenameFiles`** — handle file renames
- [ ] **`workspace/didRenameFiles`** — update references

### LSP Performance — 🔴 MISSING
- [ ] Incremental document sync (not full)
- [ ] Parallel processing of requests
- [ ] Caching for semantic tokens
- [ ] Lazy computation for expensive operations
- [ ] Request cancellation support
- [ ] Progress reporting for long operations

---

## 7. COMPILER TARGETS (MISSING)

### Target Support — 🟡 PARTIAL
- [x] WASM (via Wasmtime)
- [x] Native (via Cranelift, stub)
- [ ] **JavaScript** — compile `.ns` to `.js`
- [ ] **TypeScript** — compile to `.ts` with types
- [ ] **Python** — generate `.py` module
- [ ] **Rust** — generate `.rs` crate
- [ ] **C** — generate `.c`/`.h` files
- [ ] **C++** — generate `.cpp`/`.hpp`
- [ ] **Go** — generate `.go`
- [ ] **C#** — generate `.cs` (C#)
- [ ] **Java** — generate `.java`
- [ ] **Kotlin** — generate `.kt`
- [ ] **Swift** — generate `.swift`
- [ ] **Dart** — generate `.dart`
- [ ] **PHP** — generate `.php`
- [ ] **Ruby** — generate `.rb`
- [ ] **Lua** — generate `.lua`
- [ ] **Zig** — generate `.zig`

### Optimization Levels — 🔴 MISSING
- [ ] `--opt-level 0` (no optimizations)
- [ ] `--opt-level 1` (basic optimizations)
- [ ] `--opt-level 2` (aggressive optimizations)
- [ ] `--opt-level s` (optimize for size)
- [ ] `--opt-level z` (aggressive size)
- [ ] Function inlining heuristics
- [ ] Loop unrolling
- [ ] Dead code elimination
- [ ] Constant folding/propagation
- [ ] Tail call optimization

### Debug Info — 🔴 MISSING
- [ ] Generate debug symbols (`-g`)
- [ ] Source maps for JS/WASM targets
- [ ] DWARF support for native
- [ ] Line number mapping
- [ ] Variable name preservation

---

## 8. SECURITY & SANDBOXING (MISSING/INCOMPLETE)

### Capabilities — 🟡 PARTIAL
- [x] Filesystem allowlist
- [x] Network domain allowlist
- [x] Memory limit
- [x] Fuel (instruction) limit
- [x] Timeout
- [ ] **CPU time limit** (not wall time)
- [ ] **Disk I/O limit** (bytes read/written)
- [ ] **Network bandwidth limit**
- [ ] **Open file descriptor limit**
- [ ] **Number of subprocesses limit**
- [ ] **Environment variable allowlist**
- [ ] **System call allowlist** (seccomp-bpf)
- [ ] **Capability inheritance** across polyglot calls
- [ ] **Capability revocation** at runtime
- [ ] **Capability negotiation** between modules
- [ ] **Granular HTTP** — path, method, header allowlist
- [ ] **Rate limiting per capability**

### Sandbox Isolation — 🔴 MISSING
- [ ] Process-level isolation (fork per script)
- [ ] Container isolation (Docker/gVisor)
- [ ] Namespace isolation (Linux)
- [ ] WASM isolates (multiple stores)
- [ ] Memory isolation between scripts
- [ ] CPU core isolation/pinning
- [ ] Network namespaces
- [ ] Seccomp-bpf filtering

### Auditing — 🔴 MISSING
- [ ] Operation audit log
- [ ] File access audit trail
- [ ] Network request audit
- [ ] Subprocess audit
- [ ] Crypto operation audit
- [ ] Audit log export (JSON, CSV)
- [ ] Real-time audit streaming
- [ ] Tamper-proof audit (signature)

---

## 9. WEB PLAYGROUND (MISSING/INCOMPLETE)

### Features — 🟡 PARTIAL (basic editor exists)
- [x] Monaco editor
- [x] WASM runtime load
