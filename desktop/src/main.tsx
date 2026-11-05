import "./App.css";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { SidebarProvider } from "./components/ui/sidebar";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SidebarProvider>
      <main className="h-full w-full">
        <App />
      </main>
    </SidebarProvider>
  </React.StrictMode>,
);
