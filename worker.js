import init, { interpret } from "./pkg/motherf_wasm.js";

init();

self.onmessage = function ({ data: { code, input } }) {
  try {
    const result = interpret(code, input);
    postMessage({ result, error: null });
  } catch (error) {
    postMessage({ error, result: null });
  }
};
