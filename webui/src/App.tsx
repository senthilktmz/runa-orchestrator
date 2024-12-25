import React, { useState, useCallback } from "react";
import ReactFlow, {
    addEdge,
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
} from "reactflow";
import "reactflow/dist/style.css";
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-python";
import "ace-builds/src-noconflict/mode-sh";
import "ace-builds/src-noconflict/theme-monokai";

const initialNodes = [];
const initialEdges = [];

const App = () => {
    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
    const [selectedNode, setSelectedNode] = useState(null);
    const [currentNodeName, setCurrentNodeName] = useState("");
    const [currentNodeScript, setCurrentNodeScript] = useState("");

    const addNode = () => {
        const id = `${nodes.length + 1}`;
        const newNode = {
            id,
            data: { label: `Node ${id}`, script: "" },
            position: { x: Math.random() * 400, y: Math.random() * 400 },
        };
        setNodes((nds) => [...nds, newNode]);
    };

    const onConnect = useCallback(
        (params) => setEdges((eds) => addEdge(params, eds)),
        [setEdges]
    );

    const onNodeClick = (event, node) => {
        setSelectedNode(node);
        setCurrentNodeName(node.data.label);
        setCurrentNodeScript(node.data.script);
    };

    const handleNameChange = (e) => {
        setCurrentNodeName(e.target.value);
    };

    const handleScriptChange = (value) => {
        setCurrentNodeScript(value);
    };

    const saveNodeData = () => {
        if (selectedNode) {
            setNodes((nds) =>
                nds.map((n) =>
                    n.id === selectedNode.id
                        ? {
                            ...n,
                            data: { ...n.data, label: currentNodeName, script: currentNodeScript },
                        }
                        : n
                )
            );
            alert("Node data saved!");
        }
    };

    return (
        <div style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
            <div style={{ padding: "10px", textAlign: "center" }}>
                <button onClick={addNode}>+ Add Node</button>
            </div>
            <div style={{ display: "flex", height: "100%" }}>
                <div style={{ flex: 3 }}>
                    <ReactFlow
                        nodes={nodes}
                        edges={edges}
                        onNodesChange={onNodesChange}
                        onEdgesChange={onEdgesChange}
                        onConnect={onConnect}
                        onNodeClick={onNodeClick}
                        fitView
                        style={{ flex: 1 }}
                    >
                        <MiniMap />
                        <Controls />
                        <Background />
                    </ReactFlow>
                </div>
                {selectedNode && (
                    <div style={{ flex: 1, padding: "10px", background: "#f4f4f4" }}>
                        <h3>Edit Node</h3>
                        <label>
                            Node Name:
                            <input
                                type="text"
                                value={currentNodeName}
                                onChange={handleNameChange}
                                style={{
                                    width: "100%",
                                    marginBottom: "10px",
                                    padding: "5px",
                                    fontSize: "16px",
                                }}
                            />
                        </label>
                        <label>
                            Script:
                            <AceEditor
                                mode="python" // Change to "sh" for Bash or other supported modes
                                theme="monokai"
                                value={currentNodeScript}
                                onChange={handleScriptChange}
                                name="script_editor"
                                editorProps={{ $blockScrolling: true }}
                                setOptions={{ useWorker: false }}
                                width="100%"
                                height="300px"
                            />
                        </label>
                        <button
                            onClick={saveNodeData}
                            style={{
                                marginTop: "10px",
                                padding: "10px 15px",
                                fontSize: "16px",
                                cursor: "pointer",
                            }}
                        >
                            Save
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
};

export default App;
