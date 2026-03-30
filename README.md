# Time-Grasp

A local-first time budgeting app inspired by YNAB's envelope method, applied to hours instead of dollars. Capture, categorize, and plan your time with the same intentionality you'd use for money.

Built entirely in Rust — frontend, backend, and native shell — targeting desktop first with mobile support planned.

## Tech Stack

| Layer | Technology | Notes |
|-------|-----------|-------|
| Native Shell | Tauri 2 | Desktop/mobile runtime, IPC bridge |
| Frontend | Leptos 0.8 (Rust/WASM) | CSR, reactive stores, client-side routing |
| UI Primitives | pith-ui | Leptos port of Radix UI primitives |
| Database | SQLite via SQLx 0.8 | Compile-time verified queries, embedded migrations |
| Async | Tokio | Backend concurrency |
| Build | Trunk | Compiles Leptos to WASM, serves during dev |

## Architecture

```
Frontend (Leptos/WASM)
    |  Tauri IPC (15 commands)
Backend (Rust)
    |  SQLx
SQLite (local file)
```

### Workspace Layout

```
apps/src-tauri/     Tauri native app binary and config
crates/model/       Shared domain types (Entry, Category, CategoryGroup)
crates/backend/     Store layer, BMC, IPC handlers
crates/frontend/    Leptos components, FMC, routing
```

### Backend Layers

IPC handlers (`ipc.rs`) -> BMC (`bmc.rs`) -> Store traits (`store/traits.rs`) -> SQLx implementations

- **IPC handlers** — Thin `#[tauri::command]` functions. Construct context, delegate to BMC.
- **BMC (Backend Model Controller)** — Generic CRUD wrappers. Emit `HubEvent` on mutations.
- **Store traits** — `Creatable`, `Readable`, `Updatable`, `Deletable`, `Filterable`.
- **Store implementations** — Per-entity SQLx queries in `store/{entity}.rs`.

### Frontend Layers

- **FMC (Frontend Model Controller)** — Reactive state container using `ArcStore<KeyedVec<Uuid, T>>`. Bridges IPC and UI.
- **IPC client** — `ipc_call!` macro generates type-safe wrappers for all 15 backend commands.
- **Components** — Entry list (grouped by date), entry detail view, navigation, Radix showcase.

### Data Model

Three core tables: `category_group`, `category`, `entry`.

- UUIDs stored as BLOBs, client-generated
- STRICT mode tables with CHECK constraints
- Soft-deletion via tombstone fields (`deleted_at`, `deleted_by_user`, `deleted_by_device`, `tombstone_reason`)
- Optimistic locking via auto-incrementing `version` column (trigger-managed)
- Computed `duration` column (`end_time - start_time`) as a SQLite VIRTUAL GENERATED column
- Timestamps in epoch milliseconds

CRUD types follow a naming convention: `C_*` (create), `R_*` (read), `U_*` (update), `D_*` (delete), `*Filter` (query).

## Development

### Prerequisites

```bash
cargo install tauri-cli trunk sqlx-cli
```

### Commands

```bash
cargo tauri dev          # Run full app (Trunk + Tauri)
trunk build              # Build frontend only
cargo tauri build        # Release build
cargo test               # All tests
cargo check              # Type check
cargo clippy             # Lint
```

See the `justfile` for additional shortcuts (`just dev`, `just reload_database`, `just clean`, etc).

### Troubleshooting

If port 1420 is already in use:
```bash
pkill trunk
```

## Roadmap

### Phase 1 — Core (Local-Only MVP)

- [x] Scaffold Tauri + Leptos workspace
- [x] Integrate SQLite via SQLx with embedded migrations
- [x] Define domain model (Entry, Category, CategoryGroup)
- [x] Implement store trait layer (Creatable, Readable, Updatable, Deletable)
- [x] Full CRUD for all entities via Tauri IPC
- [x] Development data seeding
- [x] Timezone-aware time handling
- [ ] Define and wire up UI screens for data entry

### Phase 2 — UI & UX

- [ ] Iterate on and polish screen layouts for mobile
- [ ] Navigation scaffold
- [ ] Theming and design tokens
- [ ] Settings page
- [ ] Reflection workflow and reflection screens

### Phase 3 — Sync & Multi-Device

- [ ] Remote API and schema
- [ ] Background sync with batching and retry
- [ ] Conflict resolution strategy
- [ ] Account system

### Phase 4 — Observability & Operations

- [ ] Structured logging (tracing)
- [ ] Metrics (Prometheus / Grafana)
- [ ] Distributed tracing (OpenTelemetry / Tempo)
- [ ] Error tracking (Sentry)
- [ ] Performance profiling and tuning

### Phase 5 — Platform Expansion

- [ ] MacOS build
- [ ] Android build
- [ ] Windows / Linux support

## License

MIT

---

Maintainer: Adam Duvick
Platform: macOS ARM64 (primary), iOS (planned)
Language: Rust (end-to-end)
