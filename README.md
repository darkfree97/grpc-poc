### Run server

```bash
cargo run
```

### Run client

```bash
poetry install
poetry run python -m grpc_tools.protoc -I./proto --python_out=./src/ --grpc_python_out=./src/ ./proto/hello.proto
poetry run python ./src/client.py
```