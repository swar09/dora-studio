# Dora Studio Server API Reference

Base URL (default): `http://localhost:3000`

## REST Endpoints (`/api/v1`)

### Dataflows
| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/dataflows` | List all running and stopped dataflows |
| `POST` | `/api/v1/dataflows` | Start a dataflow (YAML payload expected) |
| `POST` | `/api/v1/dataflows/validate` | Validate dataflow YAML offline (Axum parsing) |
| `POST` | `/api/v1/dataflows/parse` | Parse dataflow YAML into visual Graph structure for React Flow |
| `GET` | `/api/v1/dataflows/:id` | Get details and metadata for specific dataflow |
| `DELETE` | `/api/v1/dataflows/:id` | Stop and cleanly remove a running dataflow |
| `POST` | `/api/v1/dataflows/:id/reload` | Hot-reload a running dataflow |
| `GET` | `/api/v1/dataflows/:id/nodes` | List active nodes and their status in a specific dataflow |

### Daemons & Nodes
| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/daemons` | List all connected and offline daemons |
| `GET` | `/api/v1/daemons/:machine_id` | Get system details of specific daemon |
| `GET` | `/api/v1/nodes/library` | List available generic node library elements (Python algorithms, C++ compute) |

### Telemetry (Historical)
| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/metrics` | Query stored, historical metrics from Parquet DB |
| `GET` | `/api/v1/traces` | Query stored OTLP traces |
| `GET` | `/api/v1/logs` | Query stored system/node logs |

---

## Server-Sent Events (SSE)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/events` | Stream of global cluster events (errors, starts, stops) |
| `GET` | `/api/v1/metrics/stream` | Live OTLP fan-out metrics push |

---

## WebSockets (`/ws`)

| Method | Path | Description |
|---|---|---|
| `GET` | `/ws/dataflows` | Upgrade standard HTTP connection to stream live dataflow status updates |
| `GET` | `/ws/dataflows/:id/nodes` | Upgrade to stream live per-node metrics for a dataflow |
| `GET` | `/ws/topics/:topic` | Upgrade to live-inspect custom mesh topics |
| `GET` | `/ws/logs` | Upgrade to live-stream entire cluster network logs |
