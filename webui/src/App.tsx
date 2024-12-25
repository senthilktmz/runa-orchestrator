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
            sourcePosition: "right", // Connector on the right
            targetPosition: "left",  // Connector on the left
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

    const getParentChildMap = () => {
        const map = new Map();
        edges.forEach((edge) => {
            const parent = edge.source;
            const child = edge.target;
            if (!map.has(parent)) {
                map.set(parent, { children: [], parents: [] });
            }
            if (!map.has(child)) {
                map.set(child, { children: [], parents: [] });
            }
            map.get(parent).children.push(child);
            map.get(child).parents.push(parent);
        });
        return map;
    };

    const topologicalSort = () => {
        const map = getParentChildMap();
        const visited = new Set();
        const result = [];

        const visit = (node) => {
            if (!visited.has(node)) {
                visited.add(node);
                const children = map.get(node)?.children || [];
                children.forEach(visit);
                result.push(node);
            }
        };

        nodes.forEach((node) => visit(node.id));
        return result.reverse(); // Parent-first order
    };

    const runFlow = () => {
        const sortedNodes = topologicalSort();
        console.log("Parent-First Order:", sortedNodes);
        sortedNodes.forEach((nodeId) => {
            const node = nodes.find((n) => n.id === nodeId);
            if (node) {
                console.log(`Running Node ${node.id}:`, node.data.script);
            }
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
