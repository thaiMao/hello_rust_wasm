// Importing directly from node_modules for demo purposes only.

import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
  js.main();
});
