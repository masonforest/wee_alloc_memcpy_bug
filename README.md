This is a [MCVE](https://stackoverflow.com/help/mcve) of bug
[wee_alloc#28](https://github.com/fitzgen/wee_alloc/issues/28)

To run:

    $ cargo rustc --target wasm32-unknown-unknown --lib -- -O && node run.js
    (node:98141) UnhandledPromiseRejectionWarning: LinkError: WebAssembly Instantiation: Import #0 module="env" function="memcpy" error: function import requires a callable
    ...
