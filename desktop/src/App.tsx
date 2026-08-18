// import "./App.css";

import AppWorkspace from "./components/dashboard/app-workspace";
import EventTimelineSheet from "./components/dashboard/event-timeline-sheet";

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
    <main className="flex h-screen flex-col">
      <div className="min-h-0 flex-1">
        <AppWorkspace />
      </div>
      <EventTimelineSheet />
    </main>
  );
}

export default App;
