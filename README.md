Experimenting with building a wasm based plugin system using the [CanonicalABI](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md) to provide high level strings

## Build and Run

```
./run.sh
```

## Dependencies used

#### Host

- wasmtime
  - git [#b86cba9](https://github.com/bytecodealliance/wasmtime/tree/b86cba98a975bfd5823916fa46fde81307db3b38)
- wat
  - crates.io 1.0.52
  - git [#e077155c](https://github.com/bytecodealliance/wasm-tools/tree/e077155c17107320cd8616a62fa6254a60d509b2)
- wit-component
  - crates.io 0.3.2
  - git [#e077155c](https://github.com/bytecodealliance/wasm-tools/tree/e077155c17107320cd8616a62fa6254a60d509b2)

#### Plugin

- wit-bindgen-guest-rust
  - git [#d24b97f](https://github.com/bytecodealliance/wit-bindgen/tree/d24b97fcb1378cd8f61efbfd956ca8dcb57d2db0)

## Output:

#### When using crates.io versions for `wat` and `wit-component`:

calling a simple method with no arguments and no return values works, but trying to

```
requires module: logging
requires module: test
------ calling hello-world ------
LOGGING: HELLO WORLD
------ calling hello-int ------
Error: cannot reenter component instance
```

#### When using git versions of `wat` and `wit-component`

```
requires module: logging
requires module: test
Error: decoding custom section component-type:plugin

Caused by:
    unsupported component version: 0xa (at offset 0x0)
```
