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

  if (node.name === "text_input") {
    return (
      <input
        type="text"
        placeholder={node.props.placeholder || ""}
        onChange={(e) => {
          if (node.props.id) {
            bus.setTextInput(node.props.id, e.target.value);
          }
        }}
      />
    );
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

  if (node.name === "radio_group") {
    return (
      <fieldset>
        {node.props.title && <legend>{node.props.title}</legend>}

        {node.children.map((child) => (
          <div key={child.props.value}>
            <input
              type="radio"
              id={child.props.value}
              name={child.props.label}
              value={child.props.value}
              checked={bus.getRadioGroup(node.props.id) === child.props.value}
              onChange={() => {
                bus.setRadioGroup(node.props.id, child.props.value);
              }}
            />
            <label htmlFor={child.props.value}>{child.props.label}</label>
          </div>
        ))}
      </fieldset>
    );
  }

  return <></>;
};

function App() {
  const [uiTree, setUiTree] = useState(null);
  const [textInputs, setTextInputs] = useState({});
  const [radioGroups, setRadioGroups] = useState({});
  const [sessionId, setSessionId] = useState(null);
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
            plugin_name: "simple-plugin",
            session_id: sessionId,
            action: {
              Event: {
                event,
                data: {
                  text_inputs: textInputs,
                  radio_groups: radioGroups,
                },
              },
            },
          }),
        );
      }
    },

    setTextInput: (id, value) => {
      setTextInputs((prev) => ({ ...prev, [id]: value }));
    },

    setupRadioGroup: (id, options) => {
      setRadioGroups((prev) => ({ ...prev, [id]: options[0].value }));
    },

    getRadioGroup: (id) => {
      return radioGroups[id];
    },

    setRadioGroup: (id, value) => {
      setRadioGroups((prev) => ({ ...prev, [id]: value }));
    },
  };

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8000/ws");

    socket.onopen = () => {
      setWsStatus("connected");

      socket.send(
        JSON.stringify({
          plugin_name: "simple-plugin",
          action: "Mount",
          session_id: sessionId,
        }),
      );
    };
    socket.onclose = () => setWsStatus("disconnected");
    socket.onerror = () => setWsStatus("error");
    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.UITree) {
        return setUiTree(data.UITree);
      }

      if (data.Session) {
        return setSessionId(data.Session);
      }

      if (data.Error) {
        console.error(data.Error);
      }
    };

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
