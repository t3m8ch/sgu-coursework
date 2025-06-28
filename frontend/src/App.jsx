import "./App.css";
import { useState, useEffect, useRef } from "react";

const element = (node, bus) => {
  if (node.name === "fragment") {
    return <>{node.children.map((child) => element(child, bus))}</>;
  }

  if (node.name === "row") {
    return (
      <div className="row">
        {node.children.map((child) => element(child, bus))}
      </div>
    );
  }

  if (node.name === "text") {
    const textOptionsClasses = `font-weight-${node.props.weight} text-size-${node.props.size}`;
    return <span className={textOptionsClasses}>{node.props.text}</span>;
  }

  if (node.name === "button") {
    return (
      <button
        onClick={() => {
          if (node.props.on_click_event) {
            bus.sendEvent(node.props.on_click_event);
          }
        }}
      >
        {node.children.map((child) => element(child, bus))}
      </button>
    );
  }

  return <></>;
};

function App() {
  const [uiTree, setUiTree] = useState(null);
  const [wsStatus, setWsStatus] = useState("disconnected");
  const socketRef = useRef(null);

  const bus = {
    sendEvent: (event) => {
      if (
        socketRef.current &&
        socketRef.current.readyState === WebSocket.OPEN
      ) {
        socketRef.current.send(
          JSON.stringify({
            Event: {
              plugin_name: "simple-plugin",
              event,
              data: {},
            },
          }),
        );
      }
    },
  };

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
      <div>{uiTree === null ? "Loading..." : element(uiTree, bus)}</div>
    </div>
  );
}

export default App;
