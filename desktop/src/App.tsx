import { useEffect, useState } from "react";
import { commands, ReadEvent } from "./bindings";

import "./App.css";
import { Channel } from "@tauri-apps/api/core";

const onEvent = new Channel<ReadEvent>();

function App() {
  const [prog, setProg] = useState(0);

  onEvent.onmessage = (msg) => {
    if (msg.event == "progress") {
      console.log("Progress: ", msg.data);
      setProg(msg.data.progress);
    }
    console.log("Received: ", msg.event);
  };

  useEffect(() => {
    commands.readData(onEvent);
  }, [commands]);

  return (
    <main className="container">
      <h1>Open Race Telemetry</h1>
      <p>{prog}</p>
    </main>
  );
}

export default App;
