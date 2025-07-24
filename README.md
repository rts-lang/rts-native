
## rts-native

This is an example of how the [RTS](https://github.com/rts-lang/rts) programming language 
can be embedded as a module within another project.

Integrating RTS in this way is relatively straightforward — you can easily pass variables 
or expose methods between the host environment and RTS. This approach enables more flexible 
programming without the need to recompile the entire codebase.

Such flexibility is especially important for large-scale systems like game engines, 
simulation platforms, or complex software architectures where runtime scripting capabilities are essential.

Additionally, a brief comparison is provided between RTS and other programming languages,
specifically in the context of embedding them as runtime modules (libraries) within host applications.
The focus is on their technical suitability, integration complexity, and runtime characteristics when used as 
embedded interpreters or scripting engines.

For more information about RTS, please visit the official [website](https://realtime.su/en).

| Criterion           | RTS           | Lua           | Python      | JavaScript  | WASM         | Rust Modules  |
|---------------------|---------------|---------------|-------------|-------------|--------------|---------------|
| Embeddability       | ✅ ~1MB lib   | ✅ ~250KB     | ⚠️ ~20MB    | ⚠️ ~30MB    | ⚠️ Needs VM  | ✅ Cargo/use   |
| API Simplicity      | ✅ Direct     | ✅ Simple C   | ⚠️ Complex  | ⚠️ Complex  | ⚠️ Low-level | ✅ Simple      |
| Native Host Call    | ✅ getNative  | ✅ lua_pushc  | ✅ Wrappers | ✅ JS ctx   | ✅ Import/Exp| — Native      |
| Introspection       | ✅ Built-in   | ⚠️ Limited   | ⚠️ Partial  | ⚠️ typeof  | ❌ None     | ✅ Full static |
| Extensibility       | ✅ FDS        | ⚠️ Tables    | ✅ Classes  | ✅ Objects  | ⚠️ Export API| ✅ Rust traits |
| Mutability Control  | ✅ final/...  | ❌ None      | ⚠️ Global  | ❌ None    | ⚠️ Typing   | ✅ mut/&/&mut  |
| Code Clarity        | ✅ Explicit   | ⚠️ Dynamic   | ⚠️ Dynamic | ⚠️ Dynamic | ✅ Explicit | ✅ Static      |
| Build & Distribution| ✅ .run()     | ✅ .lua      | ⚠️ Env/pip | ⚠️ NPM     | ⚠️ Complex | ✅ Cargo       |
| Modularity          | ✅ import()   | ⚠️ Manual   | ✅ Modules | ✅ ES/CJS  | ⚠️ Complex | ✅ Cargo mods  |
| Resource Usage      | ✅ Minimal    | ✅ Very low  | ❌ High    | ❌ High    | ⚠️ Moderate| ✅ Minimal     |
| Logic Type          | ✅ Ternary   | ⚠️ Bool+nil | ⚠️ Bool+None| ⚠️ Bool+null| ⚠️ Bool+NaN| ⚠️ Strict bool |
| Runtime Errors      | ✅ No exc.   | ⚠️ pcall    | ❌ exc.    | ❌ exc.    | ❌ Trap    | ⚠️ Panic      |
| Impl. Language      | ✅ Rust      | ⚠️ C        | ⚠️ C       | ⚠️ C++     | ⚠️ C/C++   | ✅ Rust       |
