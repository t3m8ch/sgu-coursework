import "./App.css";
import { useState, useEffect, useRef } from "react";

const element = (node) => {
  if (node.name === "fragment") {
    return <>{node.children.map((child) => element(child))}</>;
  }

  if (node.name === "row") {
    return (
      <div className="row">{node.children.map((child) => element(child))}</div>
    );
  }

  if (node.name === "text") {
    const textOptionsClasses = `font-weight-${node.props.weight} text-size-${node.props.size}`;
    return <span className={textOptionsClasses}>{node.props.text}</span>;
  }
};

function App() {
  const [uiTree, setUiTree] = useState(null);
  const [wsStatus, setWsStatus] = useState("disconnected");
  const socketRef = useRef(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8000/ws");

    socket.onopen = () => {
      setWsStatus("connected");

      socket.send(
        JSON.stringify({
          Mount: { plugin_name: "simple-plugin" },
        }),
      );
    };
    socket.onclose = () => setWsStatus("disconnected");
    socket.onerror = () => setWsStatus("error");
    socket.onmessage = (event) => setUiTree(JSON.parse(event.data));

    socketRef.current = socket;

    return () => {
      if (socket.readyState === WebSocket.OPEN) {
        socket.close();
      } else if (socket.readyState === WebSocket.CONNECTING) {
        socket.addEventListener("open", () => socket.close());
      }
    };
  }, []);

  return (
    <div>
      <div>{wsStatus}</div>
      <div>{uiTree === null ? "Loading..." : element(uiTree)}</div>
    </div>
  );
}

export default App;
