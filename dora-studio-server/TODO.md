# Dora Studio Server TODO

## 1. Project Setup
- [x] Initialize Cargo workspace.
- [x] Scaffold crates (`server`, `api`, `coordinator-client`, `zenoh-listener`, `otlp-listener`, `storage`, `shared`).
- [x] Configure dependencies and `tokio` binaries.

## 2. Core domain (`shared` crate)
- [x] Define `LogMessage` struct.
- [x] Define `NodeMetrics` struct.
- [x] Define `Span` / Trace models.
- [x] Define global `StudioError` types.

## 3. Storage Engine (`storage` crate)
- [x] Set up DataFusion `SessionContext`.
- [x] Define `.schema` files for Parquet (metrics, logs, spans).
- [x] Implement thread-safe `DailyWriter` schema ingestion wrappers.
- [x] Build background data retention loop (7-day/30-day sweeps).
- [x] Implement query helpers for fetching historical data.

## 4. Control Plane Client (`coordinator-client` crate)
- [x] Implement TARPC over JSON/TCP bindings for `dora_message`.
- [x] Establish connection helper to `dora-coordinator`.
- [ ] Actually invoke API wrapper functions (Start, Stop, Reload) in standard traits.

## 5. Event Data Plane (`zenoh-listener` crate)
- [ ] Open Zenoh mesh session (`session.rs`).
- [ ] Spawn `background_worker` loop to subscribe to mesh topics.
- [ ] Parse/decode binary zenoh payload logs into `LogMessage`.
- [ ] Stream ingested data to Broadcast TX channels.

## 6. Traces Data Plane (`otlp-listener` crate)
- [ ] Setup Tonic gRPC server on `:4317` (`grpc.rs`).
- [ ] Implement Protobuf translation handlers for OTLP spans.
- [ ] Stream parsed `Span` structs directly into `storage` engine.

## 7. HTTP API (`api` & `server` crates)
#### REST Endpoints
- [x] `GET /api/v1/dataflows` (List dataflows)
- [x] `POST /api/v1/dataflows` (Start dataflow)
- [x] `GET /api/v1/dataflows/:id` (Get details)
- [x] `DELETE /api/v1/dataflows/:id` (Stop/remove)
- [x] `POST /api/v1/dataflows/:id/reload` (Reload running)
- [x] `GET /api/v1/dataflows/:id/nodes` (List nodes)
- [x] `POST /api/v1/dataflows/validate` (Validate YAML)
- [x] `POST /api/v1/dataflows/parse` (Parse YAML to React Flow)
- [x] `GET /api/v1/daemons` (List daemons)
- [x] `GET /api/v1/daemons/:machine_id` (Daemon details)
- [x] `GET /api/v1/nodes/library` (List available node types)
- [x] `GET /api/v1/metrics` (Historical metrics)
- [x] `GET /api/v1/traces` (Historical traces)
- [x] `GET /api/v1/logs` (Historical logs)

#### WebSocket Endpoints
- [x] `ws:///ws/dataflows` (Live statuses)
- [x] `ws:///ws/dataflows/:id/nodes` (Live metrics per node)
- [x] `ws:///ws/topics/:topic` (Topic data inspection)
- [x] `ws:///ws/logs` (Live stream of network logs)

#### SSE Endpoints
- [x] `GET /api/v1/events` (Global component events)
- [x] `GET /api/v1/metrics/stream` (Live OTLP metrics stream)

*(Note: While the routes exist and return mock JSON/SSE data for frontend consumption, they still need to be swapped from mock to real querying from `storage` and `AppState`.)*

## 8. Server Orchestration (`server` crate)
- [x] Define application port / DB configs (`config.rs`).
- [x] Unify module routes cleanly in `routes.rs`.
- [x] Wrap Axum handlers around `tokio` threads.
- [ ] Inject `coordinator-client` connection into `AppState`.
- [ ] Inject raw `tokio::sync::broadcast` channels for WS usage inside `AppState`.
