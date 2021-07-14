# WASM Mustache Templates

This is an expermental implementation of mustache template to help proof out patterns and toolchains for producing WASM-based build tools.

## Prerequisites

1. [Rust (^1.53.0)](https://www.rust-lang.org/learn/get-started)
2. [Rust Wasm-Pack (^0.10.0)](https://rustwasm.github.io/)
3. [NodeJS (^14.17.2)](https://nodejs.org/en/)

## Build/Run Instructions

### Development build

```bash
cd mustache-wasm
wasm-pack build
cd ../minimal-web-app
npm install
npm run start
```

### Release build

```bash
cd mustache-wasm
wasm-pack build --release
cd ../minimal-web-app
npm install
npm run build
cd dist
python3 -m http.server # or `php -s localhost:8000`
```

### Compliance tests

```bash
cargo test --test spec
```

## Project Layout

- `.vscode`: Debugging Settings for Test Suite with VsCode. (Requires [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb))
- `minimal-web-app`: A minimal webpack 5 app to illustrate what's required to use the `mustache-wasm` package.
  - `dist`: The static build output for webpack app. Generated on `npm run build`.
  - `index.js`: Entry point of the minimal web app
  - `package.json`: That minimal app's package manifest. Describes the required dependencies and dev-dependencies for Wasm modules.
  - `webpack.config.js`: A webpack 5 config. Include minimal working config for Wasm modules.
- `example-web-app`: An example react app to help simulate real-world web-based work loads.
  - `index.jsx`: Entry point of the react web app
- `macros`: A Rust crate to hold any procedure macros. (As one can only export procedure macros from specialize rust-crates). Currently only used for test-generation. 
  - `src/lib.rs`: Entry point of the macros package
- `mustache-wasm`: A Rust Crate scaffolded by Wasm-Pack. It depends on the core crate, and layers on the wasm specfic details. Protecting the core crate from the `unsafe` keyword necessary for ffi.
  - `pkg`: The built `mustache-wasm` NodeJs package. Generated on `wasm-pack build`
  - `src/lib.rs`: Entry point of the wasm package
- `spec`: The set of mustache compliance files from [`mustache/spec`](https://github.com/mustache/spec), used for test generation.
- `tests/spec.rs`: Auto generates test-cases from the `spec` folder to ensure compliance.
- `src`: Source Folder for the core mustache rust crate.
  - `lib.rs`: Entry point of the core crate
- `build.rs`: Signals to rebuilds should the `spec` files ever update.
- `Cargo.toml`: Package Manifest for core rust crate.

## Rationale

Taking note of recent success like Evan You's [`vite`](https://vitejs.dev/) and skypack's [`snowpack`](https://www.snowpack.dev/) via [`esbuild`](https://esbuild.github.io/), [`deno_lint`](https://github.com/denoland/deno_lint) via [`swc`](https://swc.rs/) and the transition of [`libsass` to `Dart Sass`](https://sass-lang.com/blog/libsass-is-deprecated), once can see improving developer experience when our tools meet a high performance threshold.

However, Developing these tools can take some time due to complex nature of system programing ecosystems. Tools like rust and golang tend to fair better the C++ development, but intergrating the Node.JS and it's native can be challenge. `esbuild`, `swc`, `Dart Sass` opted for out-of-process binaries, using IPC to emit results. This introduces complexity of synchronization and a further burden tool-makers producing working binaries for operating systems that they may not have access to. (Note: While Dart can compile to JS, [using the DartVM is still prefered over NodeJS](https://github.com/sass/dart-sass/blob/main/perf.md)). 

WASM could simply this. While native code is certianly faster, having the emissions occur in-band would subsitute IPC cost with just encoding/decoding cost. (Often times, this encoding cost dimishes over time with repeated calls [thanks to JIT](https://hacks.mozilla.org/2018/10/calls-between-javascript-and-webassembly-are-finally-fast-%F0%9F%8E%89/)). Since WASM compatability would owned by an exisiting runtime, tool makers would rarely need to consider for operating systems differences. Tool makers would still have access to given runtime's ecosystem, allowing them to narrow there scope as their is less need to re-invent the wheel. And since WASM builds can be executed in the web browser, the end user can test out tools with having go through installation process.

This repos aims to proof out WASM with a simple, but non-trivial case of mustache, and see if its benifets outwiegh the downsides as well as see how approachable this strategy really is.

## TODO List

1. Handle Standalone Tags to achieve 100% non-optional compliance
2. Define a serialization format of the ast for use in template compliation
3. Measure the cost of text encoding and decoding that occurs when exchanging strings between WASM and JS
4. Implement a Webpack loader, (WASM + JS, WASM + WASI, [`clap`](https://clap.rs/) + [`child_process`](https://nodejs.org/api/child_process.html))
5. Implement a CLI package, (NodeJs w/ WASM, [Deno](https://deno.land/) w/ WASM, or Rust w/o WASM)
6. Determine an implementation strategy for Mustache Lambdas (Not required by Spec)
7. Implement Mustache Inheritance (Not required by Spec)


## Key Libraries

### Rust
1. [`serde`](https://serde.rs/): **Ser**ialization, **De**serializaion Framework for rust
    * `serde_json`, `serde_yaml`. A zero copy when possible serializiers 
2. [`nom`](https://github.com/Geal/nom)*: A zero copy parser combinator framework

#### WASM specfic

1. [`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/): Generates safely the necessarily `unsafe` FFI bindings
2. [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)*: A debugging aide for WASM modules
3. [`wee_alloc`](https://github.com/rustwasm/wee_alloc): A WASM centric memory allocator
4. [`js-sys`](https://rustwasm.github.io/wasm-bindgen/api/js_sys/): Predefined FFI bindings to Standard Javascript Objects
5. [`web-sys`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)*: Predefined FFI bindings to Web-API / DOM Objects

#### Code Generation
1. [`heck`](https://crates.io/crates/heck): A simple string casing library, useful for normalizing text to valid rust identifier when code generating.
2. [`proc-macro2`](https://docs.rs/proc-macro2/): A shim over the builtin [`proc-macro`](https://doc.rust-lang.org/book/ch19-06-macros.html#macros) to allow those structs them in typical rust enviroments, allowing for unit-testable marcos.
3. [`syn`](https://docs.rs/syn/): A set of parsers over rust-tokens streams, useful for capturing input from macros
4. [`quote`](https://docs.rs/quote/): A macro that generate rust-tokens streams via a rust-like templating DSL, useful for generating code with macros


