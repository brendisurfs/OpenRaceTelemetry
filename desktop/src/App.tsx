// import "./App.css";

import AppWorkspace from "./components/dashboard/app-workspace";

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
    <main className="h-screen">
      <AppWorkspace />
    </main>
  );
}

export default App;
