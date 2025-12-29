# Time Grasp Architecture

- [`time_grasp`](../time_grasp/index.html):
  
  This is the entry point the app. It contains a very small amount of code that sets up the app. It contains the Tauri and Trunk configuration.

  Location: `apps/src-tauri`

- [`backend`](../backend/index.html):
  
  This is all the backend business logic. It exposes ipc functions which is how the frontend can communicate with it.

  Location: `crates/backend`

- [`frontend`](../frontend/index.html):
  
  This is all the frontend components, views, and styling.

  Location: `crates/frontend`

- [`model`](../model/index.html):
  
  This is all the domain models and DTOs that are shared between the backend and frontend.

  Location: `crates/model`

---