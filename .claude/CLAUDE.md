# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

```bash
# Development (runs Trunk + Tauri together)
cargo tauri dev

# Build frontend only
trunk build

# Build release
cargo tauri build

# Run tests
cargo test                           # all tests
cargo test -p backend                # single crate
cargo test test_name                 # single test

# Check/lint
cargo check
cargo clippy

# Database migrations (auto-run on startup, but can run manually)
sqlx migrate run
```

### Troubleshooting

If you see "Address already in use" errors on port 1420:
```bash
pgrep trunk    # find orphaned trunk processes
pkill trunk    # kill them
```

## Architecture

**Stack:** Tauri 2 (native shell) + Leptos 0.8 (Rust/WASM frontend) + SQLite (SQLx)

```
Frontend (Leptos/WASM)
    ↓ Tauri IPC
Backend (Rust)
    ↓
SQLite
```

### Workspace Structure

- `crates/model/` - Shared domain types used by frontend and backend
- `crates/backend/` - Persistence, business logic, Tauri IPC handlers
- `crates/frontend/` - Leptos reactive UI
- `apps/src-tauri/` - Tauri native app configuration

### Model Naming Convention

CRUD types use prefixes to indicate their purpose:
- `C_*` / `*ForCreate` - Create/insert payloads
- `R_*` - Read/view models (what you get back from queries)
- `U_*` / `*ForUpdate` - Update payloads with optional fields
- `D_*` / `*ForDelete` - Delete/tombstone payloads
- `*Filter` - Query filter parameters

### Backend Layer Pattern

IPC handlers (`ipc.rs`) → BMC (`bmc.rs`) → Store traits (`store/traits.rs`) → SQLx implementations

- **IPC handlers:** Thin Tauri `#[command]` functions that construct `Ctx` and delegate to BMC
- **BMC (Backend Model Controller):** Generic CRUD wrappers that call store operations and emit hub events
- **Store traits:** `Creatable<T>`, `Readable<T>`, `Updatable<T>`, `Deletable<T>`, `Filterable`
- **Store implementations:** Per-entity SQLx implementations in `store/{entity}.rs`

### Frontend State Management

- **FMC (Frontend Model Controller):** `crates/frontend/src/fmc.rs` - Reactive state container
- Uses `reactive_stores::ArcStore<KeyedVec<Uuid, T>>` for collections
- `Crud<T>` trait implemented for each entity type with optimistic updates
- IPC calls wrapped in `crates/frontend/src/ipc.rs` using `ipc_call!` macro

### Key Types

- `EpochMillis` - Timestamp wrapper (milliseconds since epoch)
- `DurationMillis` - Signed duration in milliseconds
- `FieldUpdate<T>` - Distinguishes "unchanged" vs "set to None" in updates
- `KeyedVec<K, T>` - Vec wrapper for reactive keyed iteration

### Database Conventions

- UUIDs stored as BLOBs (client-supplied)
- STRICT tables
- Soft-deletion via tombstone semantics (`deleted_at`, `deleted_by_*`, `tombstone_reason`)
- Triggers handle `version` increment and `updated_at` timestamps
- Compile-time verified queries via `sqlx::query!`
