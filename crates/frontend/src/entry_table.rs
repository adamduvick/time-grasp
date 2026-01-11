use ::chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos_struct_table::*;

use model::{Entry, EntryFilter, Uuid};

use crate::ipc;

/// Classes:
/// - grid-container
/// - grid-body
/// - grid-header
/// - grid-header-cell + grid-header-cell-<column-name>
/// - grid-row
/// - grid-row-cell + grid-cell-<column-name>
///
/// Meta:
/// - data-column: <column-name>
///

#[derive(TableRow, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[table(impl_vec_data_provider, sortable)]
pub struct EntryRow {
    uuid: Uuid,
    name: String,
    note: String,
}

impl From<&Entry> for EntryRow {
    fn from(entry: &Entry) -> Self {
        Self {
            uuid: entry.id,
            name: entry.name.clone(),
            note: entry.note.clone().unwrap_or_default(),
        }
    }
}

#[component]
pub fn EntryTable() -> impl IntoView {
    // Kick off the async work (use LocalResource on wasm — futures aren't `Send`)
    let entries = LocalResource::new(|| async move {
        ipc::list_entry(EntryFilter::new()).await.map(|entries| {
            entries
                .into_iter()
                .map(|entry| EntryRow::from(&entry))
                .collect::<Vec<_>>()
        })
    });

    view! {
            // Catches panics *and* Resource errors thrown via `?`
        <ErrorBoundary
            fallback=|errs| view! {
                <div class="error">
                    <h3>"Something went wrong"</h3>
                    <pre>{move || format!("{:#?}", errs.get())}</pre>
                </div>
            }
        >
        <Suspense fallback=|| view! { <p>"Loading…"</p> }>
            {move || {
                // `entries.get()` is Option<Result<...>>
                match entries.get() {
                    None => view! { }.into_any(), // still loading; Suspense covers this
                    Some(Err(e)) => view! { <ErrorView error=e /> }.into_any(),
                    Some(Ok(v)) => view! {
                        <table>
                        <TableContent rows=v scroll_container="body" display_strategy=DisplayStrategy::InfiniteScroll />
                        </table>
                    }.into_any(),
            }
            }}
        </Suspense>
        </ErrorBoundary>
    }
}

#[component]
fn ErrorView(error: crate::error::Error) -> impl IntoView {
    view! {
        <div class="error">
            <h3>"Failed to load entries"</h3>
            <pre>{format!("{error:?}")}</pre>
        </div>
    }
}

enum EntryColumn {
    Name,
    Note,
    Category,
    StartTime,
    Duration,
}

impl EntryColumn {
    fn label(&self) -> &'static str {
        match self {
            EntryColumn::Name => "Name",
            EntryColumn::Note => "Note",
            EntryColumn::Category => "Category",
            EntryColumn::StartTime => "Start Time",
            EntryColumn::Duration => "Duration",
        }
    }
}
