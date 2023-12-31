import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import "./styles.css";
import "./configs/i18n";

import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

// eslint-disable-next-line @typescript-eslint/non-nullable-type-assertion-style
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
