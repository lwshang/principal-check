This repo is for illustration purpose.

It proves that annotating `Principal` with `#[repr(packed)]` is not necessary.
With or without the annotation, the struct `Principal` always have the same size on wasm32 platform.

To run the test, firstly install `wasm-pack` tool as described in [site](https://rustwasm.github.io/wasm-pack/installer/).

Then in this project, run:

```
wasm-pack test --node
```