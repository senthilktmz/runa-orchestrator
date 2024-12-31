
#### Build Orchestrator
```bash
cargo build
```


#### Run Orchestrator
```bash
cargo run -- --work-dir workdir01 --port 9191
```

#### Build task_agent
```bash
cargo build --features server_type_task_agent
```


#### Run task_agent
```bash
cargo run --features server_type_task_agent -- --work-dir=/tmp/ssks89 --port=12050
```

