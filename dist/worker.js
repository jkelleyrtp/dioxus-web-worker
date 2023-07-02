import init from "/./assets/dioxus/dioxus.js";

init("/./assets/dioxus/dioxus_bg.wasm").then(wasm => {
  wasm.start_webworker();
});
