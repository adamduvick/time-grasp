Potential future deps:

- [Sea Query](https://crates.io/crates/sea-query): dynamic query building for more pure rust querying
  - not needed right now, but may be good for complex filters
- [ModQL](https://crates.io/crates/modql): works with Sea Query to add dynamic filtering
  - not needed right now, but may be good for complex filters
- [Leptos Fetch](https://crates.io/crates/leptos-fetch) or [Leptos Query](https://crates.io/crates/leptos-fetch)
  - provides resource caching a deduplication so that components can subscribe to uuids and be told to update when the corresponding resource/entity changes