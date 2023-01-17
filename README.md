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
- wit-component
  - crates.io 0.3.2

#### Plugin

- wit-bindgen-guest-rust
  - git [#d24b97f](https://github.com/bytecodealliance/wit-bindgen/tree/d24b97fcb1378cd8f61efbfd956ca8dcb57d2db0)
