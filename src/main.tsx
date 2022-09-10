import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

import initWasm, { start } from "../wasm/pkg/wasm";

async function init() {
  await initWasm();

  start();

  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}

init();
