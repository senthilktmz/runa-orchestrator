
#### Build Orchestrator
```bash
cargo build --features server_type_orchestrator
```


#### Run Orchestrator
```bash
cargo run --features server_type_orchestrator -- --work-dir=/tmp/ssks89 --port=9829
```

#### Build task_agent
```bash
cargo build --features server_type_task_agent
```


#### Run task_agent
```bash
cargo run --features server_type_task_agent -- --work-dir=/tmp/ssks89 --port=12050
```

