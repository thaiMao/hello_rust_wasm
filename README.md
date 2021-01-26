Building the NPM package (inside pkg directory)

$wasm-pack build

Making a package available to NPM (output an NPM package written in Rust, but compiled to WASM)

$ cd pkg
$ npm link

Using the package on the Web

$ cd ../..
$ mkdir site
$ cd site
$ npm link hello-wasm

Add package to dependency list in package.json.

Start dev server (from site directory)

$ npm install
$ npm run serve

Go to http://localhost:8080
