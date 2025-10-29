# 🕰️ Time-Grasp

> A YNAB-style **time budgeting app** for people who want to track where their hours actually go.

Time-Grasp helps you **capture, categorize, and plan your time** with the same intention you’d use for your money.  
Built with **Rust + Leptos + Tauri**, it’s fast, offline-capable, and privacy-respecting.

---

## 🚀 Overview

- **Product:** Local-first, single-user time budgeting/tracking app (MVP)
- **Vision:** Eventually sync across devices; analyze trends; support goals/targets
- **Design Goals:**  
  - Snappy 60 Hz UX feel  
  - Local database with future server sync  
  - Clean, reactive UI  
  - High maintainability and testability  

---

## 🧱 Tech Stack

### App Shell
| Layer    | Tech                   | Purpose                                         |
| -------- | ---------------------- | ----------------------------------------------- |
| Runtime  | **Tauri**              | Native desktop/mobile shell, IPC bridge         |
| Frontend | **Leptos (Rust/WASM)** | Reactive UI framework                           |
| Builder  | **Trunk**              | Builds Leptos → static assets for Tauri         |
| Mobile   | **Tauri iOS**          | Xcode project; iOS builds via Rust+Swift bridge |

---

### Backend (Rust)
| Component     | Library                     | Description                                 |
| ------------- | --------------------------- | ------------------------------------------- |
| Database      | **SQLite + SQLx**           | Local persistent store                      |
| Migrations    | `sqlx::migrate!()`          | Runs embedded migrations at startup         |
| IPC Gateway   | Custom                      | Central CRUD interface (frontend ↔ backend) |
| Async Runtime | **Tokio**                   | Concurrency                                 |
| Observability | **OpenTelemetry + Tracing** | Instrumentation for metrics/logs/traces     |

---

### Observability Stack
| Layer   | Service                        | Notes                                   |
| ------- | ------------------------------ | --------------------------------------- |
| Metrics | **Prometheus → Grafana Cloud** | Free-tier metrics collection            |
| Traces  | **Grafana Tempo**              | Linked to OpenTelemetry traces          |
| Logs    | **Grafana Loki**               | Structured backend logs                 |
| Errors  | **Sentry**                     | Frontend/backend crash + issue tracking |

---

## 🧩 Data Model (Normalized)

| Table             | Fields                                     | Description                                  |
| ----------------- | ------------------------------------------ | -------------------------------------------- |
| **Entry**         | `id, payee, start, end, category_id, memo` | A single time entry (duration = end − start) |
| **Category**      | `id, name, group_id, notes`                | A specific “budget” line for time            |
| **CategoryGroup** | `id, name, notes`                          | Grouping of categories (organization only)   |

> **Key decision:** Local DB uses `BIGINT` primary keys for speed and simplicity.  
> Server will maintain its own authoritative keys once sync is introduced.

---

## 🪄 Architecture

Frontend (Leptos)
↓  [Tauri IPC]
Backend (Rust)
↓
SQLite (SQLx)

**Data Flow Pattern (#1):**
1. Frontend performs CRUD → backend gateway.  
2. Backend updates local DB (authoritative).  
3. Later, backend batches/syncs with remote server.  
4. Backend tracks and retries failed syncs.

---

## 🧠 UX / MVP Slice

| Screen          | Description                                                     |
| --------------- | --------------------------------------------------------------- |
| **Entry List**  | Groups entries by date; each row shows name, duration, category |
| **FAB**         | Floating Action Button → opens modal form                       |
| **Modal Form**  | Fields: date, duration, name, memo, category                    |
| **Feedback**    | Optimistic insert + toast confirmation                          |
| **Persistence** | Data survives restart; migration-safe                           |

---

## 🎨 UI System

Component hierarchy inspired by atomic design:

- **Atoms:** Buttons, inputs, labels, icons, typography  
- **Molecules:** Entry row, category selector, modal form  
- **Organisms:** Entry list, entry editor, navigation scaffold  
- **Pages:** Main view (time entries), settings (later)

Design tokens will drive colors, spacing, and typography for cross-platform consistency.

---

## 🔐 Security & Permissions

- Tauri sandbox with minimal privileges  
- CSP disabled (`null`) during development for iteration  
- File system access TBD (export/import phase)  

---

## 🛠️ Development Setup

```bash
# 1. Install prerequisites
cargo install tauri-cli trunk sqlx-cli

# 2. Build frontend
trunk build

# 3. Run in dev mode
tauri dev

# 4. Run migrations (auto-run on startup)
sqlx migrate run
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## To Run

Template created! To get started run:
```shell
cd time-grasp
cargo tauri android init
cargo tauri ios init
```

For Desktop development, run:
```shell
cargo tauri dev
```

For Android development, run:
```shell
cargo tauri android dev
```

For iOS development, run:
```shell
cargo tauri ios dev
```

⸻

🗺️ Roadmap

✅ Phase 1 — MVP (Local-Only)
	•	Scaffold Tauri + Leptos app
	•	Integrate SQLx + SQLite
	•	CRUD for Entries/Categories
	•	Grouped Entry List + FAB form
	•	Persistent local DB

🏗️ Phase 2 — UX Polish
	•	Component library (atoms → organisms)
	•	Theming + design tokens
	•	Basic analytics (weekly totals, charts)
	•	Settings UI

🌐 Phase 3 — Sync & Cloud
	•	Define remote schema & API
	•	Implement backend batching & sync
	•	Add account system (multi-device)

🪶 Phase 4 — Refinement
	•	Observability dashboards
	•	Error telemetry → Grafana/Sentry
	•	Performance tuning

⸻

⚙️ Open Questions
	•	Server language/framework (Rust Axum? Go? TBD)
	•	Local→server ID mapping strategy
	•	Native date/time picker styling across OS
	•	Sync conflict resolution

⸻

🧾 License

MIT (default; may change for commercial release)

⸻

Maintainer: @time-grasp
Primary Dev Environment: macOS ARM64
Language: Rust
Build Target: macOS, iOS (soon), Windows/Linux (later)

---
