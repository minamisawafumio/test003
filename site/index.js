const js = import("./node_modules/@minamisawafumio/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});