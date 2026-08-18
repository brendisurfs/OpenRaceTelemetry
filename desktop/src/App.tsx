// import "./App.css";
// import { Channel } from "@tauri-apps/api/core";

import ViewerSquare from "./components/viewer-square";

// const onEvent = new Channel<ReadEvent>();

function App() {
  // onEvent.onmessage = (msg) => {
  //   if (msg.event == "progress") {
  //     console.log("Progress: ", msg.data);
  //     setProg(msg.data.progress);
  //   }
  //   console.log("Received: ", msg.event);
  // };

  // useEffect(() => {
  //   commands.readData(onEvent);
  // }, [commands]);

  return (
    <main className="flex flex-col">
      <ViewerSquare />
    </main>
  );
}

export default App;
