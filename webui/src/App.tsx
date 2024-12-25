import React, { useState, useCallback } from "react";
import ReactFlow, {
    addEdge,
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
    MarkerType,
} from "reactflow";
import "reactflow/dist/style.css";
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-python";
import "ace-builds/src-noconflict/mode-sh";
import "ace-builds/src-noconflict/mode-java";
import "ace-builds/src-noconflict/mode-c_cpp";
import "ace-builds/src-noconflict/mode-rust";
import "ace-builds/src-noconflict/mode-json";
import "ace-builds/src-noconflict/mode-xml";
import "ace-builds/src-noconflict/mode-yaml";
import "ace-builds/src-noconflict/theme-monokai";

const initialNodes = [];
const initialEdges = [];

const App = () => {
    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
    const [selectedNode, setSelectedNode] = useState(null);
    const [currentNodeName, setCurrentNodeName] = useState("");
    const [currentNodeScript, setCurrentNodeScript] = useState("");
    const [currentNodeType, setCurrentNodeType] = useState("python");
    const [isEditorPopupOpen, setIsEditorPopupOpen] = useState(false);

    const addNode = () => {
        const id = `${nodes.length + 1}`;
        const newNode = {
            id,
            data: { label: `Node ${id}`, script: "", type: "python" },
            position: { x: Math.random() * 400, y: Math.random() * 400 },
            sourcePosition: "right",
            targetPosition: "left",
        };
        setNodes((nds) => [...nds, newNode]);
    };

    const onConnect = useCallback(
        (params) => setEdges((eds) => addEdge({ ...params, markerEnd: { type: MarkerType.Arrow } }, eds)),
        [setEdges]
    );

    const onNodeClick = (event, node) => {
        setSelectedNode(node);
        setCurrentNodeName(node.data.label);
        setCurrentNodeScript(node.data.script);
        setCurrentNodeType(node.data.type);
    };

    const handleNameChange = (e) => {
        setCurrentNodeName(e.target.value);
    };

    const handleScriptChange = (value) => {
        setCurrentNodeScript(value);
    };

    const handleTypeChange = (e) => {
        setCurrentNodeType(e.target.value);
    };

    const saveNodeData = () => {
        if (selectedNode) {
            setNodes((nds) =>
                nds.map((n) =>
                    n.id === selectedNode.id
                        ? {
                            ...n,
                            data: { ...n.data, label: currentNodeName, script: currentNodeScript, type: currentNodeType },
                        }
                        : n
                )
            );
            alert("Node data saved!");
        }
    };

    const toggleEditorPopup = () => {
        setIsEditorPopupOpen(!isEditorPopupOpen);
    };

    const runFlow = () => {
        console.log("Running the flow...");
        // Example: Log the scripts and types in parent-first order.
        edges.forEach((edge) => {
            const source = nodes.find((n) => n.id === edge.source);
            const target = nodes.find((n) => n.id === edge.target);
            console.log(`${source?.data.label} (parent) -> ${target?.data.label} (child)`);
        });
    };

    return (
        <div style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
            <div style={{ padding: "10px", textAlign: "center" }}>
                <button onClick={addNode}>+ Add Node</button>
                <button onClick={runFlow} style={{ marginLeft: "10px" }}>
                    Run
                </button>
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
                            Script Type:
                            <select
                                value={currentNodeType}
                                onChange={handleTypeChange}
                                style={{
                                    width: "100%",
                                    marginBottom: "10px",
                                    padding: "5px",
                                    fontSize: "16px",
                                }}
                            >
                                <option value="python">Python</option>
                                <option value="sh">Bash</option>
                                <option value="java">Java</option>
                                <option value="c_cpp">C/C++</option>
                                <option value="rust">Rust</option>
                                <option value="json">JSON</option>
                                <option value="xml">XML</option>
                                <option value="yaml">YAML</option>
                            </select>
                        </label>
                        <label>
                            Script:
                            <AceEditor
                                mode={currentNodeType}
                                theme="monokai"
                                value={currentNodeScript}
                                onChange={handleScriptChange}
                                name="script_editor"
                                editorProps={{ $blockScrolling: true }}
                                setOptions={{ useWorker: false }}
                                width="100%"
                                height="150px"
                            />
                        </label>
                        <button
                            onClick={toggleEditorPopup}
                            style={{
                                marginTop: "10px",
                                padding: "10px 15px",
                                fontSize: "16px",
                                cursor: "pointer",
                            }}
                        >
                            Expand Editor
                        </button>
                        <button
                            onClick={saveNodeData}
                            style={{
                                marginTop: "10px",
                                padding: "10px 15px",
                                fontSize: "16px",
                                marginLeft: "10px",
                                cursor: "pointer",
                            }}
                        >
                            Save
                        </button>
                    </div>
                )}
            </div>

            {/* Popup Editor */}
            {isEditorPopupOpen && (
                <div
                    style={{
                        position: "fixed",
                        top: "50%",
                        left: "50%",
                        transform: "translate(-50%, -50%)",
                        width: "80%",
                        height: "70%",
                        backgroundColor: "#fff",
                        border: "1px solid #ccc",
                        boxShadow: "0 0 10px rgba(0, 0, 0, 0.3)",
                        zIndex: 1000,
                        padding: "20px",
                    }}
                >
                    <h3>Expanded Editor</h3>
                    <AceEditor
                        mode={currentNodeType}
                        theme="monokai"
                        value={currentNodeScript}
                        onChange={handleScriptChange}
                        name="popup_script_editor"
                        editorProps={{ $blockScrolling: true }}
                        setOptions={{ useWorker: false }}
                        width="100%"
                        height="calc(100% - 50px)"
                    />
                    <button
                        onClick={toggleEditorPopup}
                        style={{
                            marginTop: "10px",
                            padding: "10px 15px",
                            fontSize: "16px",
                            cursor: "pointer",
                        }}
                    >
                        Close
                    </button>
                </div>
            )}
            {/* Overlay for Popup */}
            {isEditorPopupOpen && (
                <div
                    onClick={toggleEditorPopup}
                    style={{
                        position: "fixed",
                        top: 0,
                        left: 0,
                        width: "100%",
                        height: "100%",
                        backgroundColor: "rgba(0, 0, 0, 0.5)",
                        zIndex: 999,
                    }}
                />
            )}
        </div>
    );
};

export default App;
