Building the NPM package (inside pkg directory)

$wasm-pack build

Making a package available to NPM (output an NPM package written in Rust, but compiled to WASM)

$ cd pkg
$ npm link
