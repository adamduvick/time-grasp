use leptos::prelude::*;
use model::*;
use std::{borrow::Cow, clone, marker::PhantomData, ops::Deref, rc::Rc, sync::Arc};

// ----- IDs & keys -----
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColumnId(pub &'static str);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RowId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RowKey(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CellKey {
    pub row: RowKey,
    pub col: ColumnId,
}

pub trait RowLike: Clone + Send + Sync + 'static {}
impl<T> RowLike for T where T: Clone + Send + Sync + 'static {}

// ----- Column schema -----
pub type ViewFn0 = Arc<dyn Fn() -> AnyView + Send + Sync + 'static>;
pub type ViewFnRow<Row> = Arc<dyn Fn(&Row) -> AnyView + Send + Sync + 'static>;
pub type EditViewFnRow<Row> = Arc<dyn Fn(&Row) -> AnyView + Send + Sync + 'static>;

#[derive(Clone)]
pub struct ColumnDef<Row: RowLike> {
    pub id: ColumnId,
    pub header: ViewFn0,
    pub cell: ViewFnRow<Row>,
    pub editor: Option<EditViewFnRow<Row>>,
    pub sortable: bool,
    pub class: &'static str,
    pub header_class: &'static str,
}

#[derive(Clone)]
pub struct GridSchema<Row: RowLike> {
    pub columns: Vec<ColumnDef<Row>>,
    pub column_order: Vec<ColumnId>,
}

impl<Row: RowLike> GridSchema<Row> {
    pub fn ordered_columns(&self) -> Vec<ColumnDef<Row>> {
        // For sketches: stable order by column_order, fallback to columns vec order
        if self.column_order.is_empty() {
            return self.columns.clone();
        }
        let mut out = Vec::with_capacity(self.columns.len());
        for id in &self.column_order {
            if let Some(c) = self.columns.iter().find(|c| c.id == *id) {
                out.push(c.clone());
            }
        }
        out
    }
}

// ----- State & events -----
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortSpec {
    pub col: ColumnId,
    pub dir: SortDir,
}

#[derive(Clone, Debug, Default)]
pub struct GridState {
    pub sort: Option<SortSpec>,
    pub active: Option<CellKey>,
    pub editing: Option<CellKey>,
    pub selected_row: Option<RowId>,
    pub use_activedescendant: bool,
}

#[derive(Clone, Debug)]
pub enum GridEvent {
    SortBy(ColumnId),
    SetActive(CellKey),
    BeginEdit(CellKey),
    CommitEdit(CellKey),
    CancelEdit,
    SelectRow(RowId),
    EnsureVisible(CellKey), // for virtualizer hook
}

fn reduce(state: &mut GridState, ev: &GridEvent) {
    match ev {
        GridEvent::SortBy(col) => {
            state.sort = match &state.sort {
                Some(s) if s.col == *col => Some(SortSpec {
                    col: col.clone(),
                    dir: if s.dir == SortDir::Asc {
                        SortDir::Desc
                    } else {
                        SortDir::Asc
                    },
                }),
                _ => Some(SortSpec {
                    col: col.clone(),
                    dir: SortDir::Asc,
                }),
            };
        }
        GridEvent::SetActive(cell) => state.active = Some(cell.clone()),
        GridEvent::BeginEdit(cell) => state.editing = Some(cell.clone()),
        GridEvent::CommitEdit(cell) => {
            state.editing = None;
            state.active = Some(cell.clone());
        }
        GridEvent::CancelEdit => state.editing = None,
        GridEvent::SelectRow(id) => state.selected_row = Some(id.clone()),
        GridEvent::EnsureVisible(_) => {}
    }
}

// ----- Controller & context -----
#[derive(Clone)]
pub struct GridController<Row: RowLike> {
    pub dispatch: Arc<dyn Fn(GridEvent) + Send + Sync + 'static>,
    pub rows: ReadSignal<Vec<Row>>,
    pub schema: Arc<GridSchema<Row>>,
    pub row_id: Arc<dyn Fn(&Row) -> RowId + Send + Sync + 'static>,
}

#[derive(Clone)]
pub struct GridCtx<Row: RowLike> {
    pub schema: Arc<GridSchema<Row>>,
    pub rows: ReadSignal<Vec<Row>>,
    pub state: RwSignal<GridState>,
    pub ctrl: GridController<Row>,
    pub grid_id: &'static str,
}

// ----- Helpers -----

/// Build stable DOM id for a cell (needed for aria-activedescendant)
fn cell_dom_id(grid_id: &str, cell: &CellKey) -> String {
    format!("{grid_id}__r{}__c{}", cell.row.0, cell.col.0)
}

// =====================================================================================
// Components
// =====================================================================================

#[component]
pub fn GridRoot<Row: RowLike>(
    /// Stable id for aria-activedescendant targets
    grid_id: &'static str,
    aria_label: &'static str,
    schema: GridSchema<Row>,
    rows: ReadSignal<Vec<Row>>,
    row_id: Arc<dyn Fn(&Row) -> RowId + Send + Sync + 'static>,
    /// Whether to keep DOM focus on root and move active cell via aria-activedescendant
    use_activedescendant: bool,
    children: Children,
) -> impl IntoView {
    let schema = Arc::new(schema);
    let state = RwSignal::new(GridState {
        use_activedescendant,
        ..Default::default()
    });

    // dispatch: reducer + room for plugin hooks later
    let dispatch: Arc<dyn Fn(GridEvent) + Send + Sync + 'static> = {
        let state = state.clone();
        Arc::new(move |ev| state.update(|s| reduce(s, &ev)))
    };

    let ctrl = GridController {
        dispatch: dispatch.clone(),
        rows,
        schema: schema.clone(),
        row_id: row_id.clone(),
    };

    provide_context(GridCtx {
        schema,
        rows,
        state,
        ctrl,
        grid_id,
    });

    // Root: focusable if activedescendant model (recommended for virtualization)
    let tabindex = move || if use_activedescendant { 0 } else { -1 };

    // Active descendant id is a *string id* of the active cell element
    let activedesc = move || {
        if !use_activedescendant {
            return None;
        }
        let ctx = use_context::<GridCtx<Row>>().unwrap();
        ctx.state
            .get()
            .active
            .map(|cell| cell_dom_id(ctx.grid_id, &cell))
    };

    view! {
        <div
            id=grid_id
            class="grid-root"
            role="grid"
            aria-label=aria_label
            tabindex=tabindex
            aria-activedescendant=move || activedesc().unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn GridHeader<Row: RowLike>(_marker: PhantomData<Row>) -> impl IntoView {
    let ctx = use_context::<GridCtx<Row>>().expect("GridHeader must be inside GridRoot");

    let cols = move || ctx.schema.ordered_columns();

    view! {
        <div class="grid-header" role="rowgroup">
            <div class="grid-row grid-header-row" role="row" >
                <For
                    each=cols
                    key=|c| c.id.clone()
                    let:col
                >
                    <GridHeaderCell col=col />
                    // {grid_header_cell(col)}
                </For>
            </div>
        </div>
    }
}

#[component]
fn GridHeaderCell<Row: RowLike>(col: ColumnDef<Row>) -> impl IntoView {
    let ctx = use_context::<GridCtx<Row>>().unwrap();
    let col_id = col.id;
    let col_sortable = col.sortable;
    let state = ctx.state.clone();
    let dispatch = ctx.ctrl.dispatch;

    let aria_sort = {
        let col_id = col_id.clone();
        move || match state.get().sort {
            Some(SortSpec { col: active, dir }) if active == col_id => match dir {
                SortDir::Asc => "ascending",
                SortDir::Desc => "descending",
            },
            _ => "none",
        }
    };

    // Header cells are interactive if sortable
    let tabindex = move || if col.sortable { 0 } else { -1 };

    let on_activate = {
        let col_id = col_id.clone();
        move || {
            if col_sortable {
                (dispatch)(GridEvent::SortBy(col_id.clone()));
            }
        }
    };
    let on_activate2 = on_activate.clone();

    view! {
        <div
            class=move || format!("grid-cell grid-header-cell {} {}", col.class, col.header_class)
            role="columnheader"
            aria-sort=aria_sort
            tabindex=tabindex
            data-column=col_id.0.to_string()
            on:click=move |_| on_activate()
            on:keydown=move |ev| {
                if !col.sortable { return; }
                let k = ev.key();
                if k == "Enter" || k == " " {
                    ev.prevent_default();
                    on_activate2();
                }
            }
        >
            {(col.header)()}
        </div>
    }
}

#[component]
pub fn GridBody<Row: RowLike>(_marker: PhantomData<Row>) -> impl IntoView {
    let ctx = use_context::<GridCtx<Row>>().expect("GridBody must be inside GridRoot");
    let cols = Signal::derive(move || ctx.schema.ordered_columns());

    // Note: this is non-virtualized body. A virtualized version would:
    // - render only a range of row indices
    // - provide aria-rowcount + aria-rowindex
    view! {
        <div class="grid-body-container">
            <div class="grid-body" role="rowgroup">
                <For
                    each={move || ctx.rows.get().into_iter().enumerate().collect::<Vec<_>>()}
                    key={move |(_, r)| (ctx.ctrl.row_id)(r).0.clone()} // stable identity
                    let:item
                >
                        {
                            let (idx, row) = item;
                            view! {
                                <GridRow
                                    row_index=RowKey(idx)
                                    row=row
                                    cols=cols
                                />
                                // {grid_row(RowKey(idx), row, cols)}
                            }
                        }
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn GridRow<Row: RowLike>(
    row_index: RowKey,
    row: Row,
    cols: Signal<Vec<ColumnDef<Row>>>,
) -> impl IntoView {
    let ctx = use_context::<GridCtx<Row>>().unwrap();
    let rid = (ctx.ctrl.row_id)(&row);
    let row = Arc::new(row);

    let is_selected = {
        let rid = rid.clone();
        move || ctx.state.get().selected_row.as_ref() == Some(&rid)
    };
    let is_selected2 = is_selected.clone();

    // Row is focusable only if you're doing "row focus" UX. For grid UX, focus stays on cells/root.
    // We'll keep it unfocusable by default.
    view! {
        <div
            class=move || if is_selected() { "grid-row is-selected" } else { "grid-row" }
            role="row"
            aria-selected=move || if is_selected2() { "true" } else { "false" }
            data-row-id=rid.clone().0.to_string()
            on:click=move |_| (ctx.ctrl.dispatch)(GridEvent::SelectRow(rid.clone()))
        >
            <For
                each=move || cols.get()
                key=|c| c.id.clone()
                let:col
            >
                <GridCell row_index=row_index.clone() col=col row=row.clone() />
                // {grid_cell(row_index.clone(), col, row.clone())}
            </For>
        </div>
    }
}

#[component]
pub fn GridCell<Row: RowLike>(
    row_index: RowKey,
    col: ColumnDef<Row>,
    row: Arc<Row>,
) -> impl IntoView {
    let ctx = use_context::<GridCtx<Row>>().unwrap();

    // Pull out the few things we need so we don't accidentally move `ctx` into one closure.
    let state = ctx.state; // signal handle (Copy-ish)
    let dispatch = ctx.ctrl.dispatch.clone(); // Arc/dyn Fn - cloneable
    let grid_id = ctx.grid_id;

    let key = Arc::new(CellKey {
        row: row_index,
        col: col.id.clone(),
    });
    let dom_id = cell_dom_id(ctx.grid_id, &key);

    // Split ColumnDef so we don't keep capturing `col` by move everywhere.
    let col_id = col.id; // &'static str -> Copy
    let col_class = col.class; // &'static str -> Copy
    let cell_fn = col.cell.clone(); // Arc<dyn Fn(&Row)->View> (or Rc) -> cloneable
    let editor_fn = col.editor.clone(); // Option<Arc<dyn Fn(&Row)->View>>
    let is_editable = editor_fn.is_some();

    // These closures need `key` and `state` repeatedly.
    let is_active = {
        let key = key.clone();
        move || state.get().active.as_ref() == Some(&*key)
    };
    let is_editing = {
        let key = key.clone();
        move || state.get().editing.as_ref() == Some(&*key)
    };

    let tabindex = {
        let is_active = is_active.clone();
        move || {
            if state.get().use_activedescendant {
                -1
            } else if is_active() {
                0
            } else {
                -1
            }
        }
    };

    let set_active = {
        let key = key.clone();
        let dispatch = dispatch.clone();
        move || dispatch(GridEvent::SetActive((*key).clone()))
    };

    let begin_edit = {
        let key = key.clone();
        let dispatch = dispatch.clone();
        move || dispatch(GridEvent::BeginEdit((*key).clone()))
    };

    // let key_for_closure = key.clone();
    // let ctx_state = ctx.state.clone();
    // let is_active = move || ctx_state.get().active.as_ref() == Some(&key_for_closure);

    // let key_for_closure = key.clone();
    // let ctx_state = ctx.state.clone();
    // let is_editing = move || ctx_state.get().editing.as_ref() == Some(&key_for_closure);

    // // Roving tabindex model (when NOT using activedescendant):
    // // - only active cell gets tabindex=0
    // // - all others -1
    // let is_active_for_closure = is_active.clone();
    // let tabindex = move || {
    //     if ctx.state.get().use_activedescendant {
    //         -1
    //     } else if is_active_for_closure() {
    //         0
    //     } else {
    //         -1
    //     }
    // };

    // let key_for_closure = key.clone();
    // let dispatch = ctx.ctrl.dispatch.clone();
    // let set_active = move || (dispatch)(GridEvent::SetActive(key_for_closure));

    // let key_for_closure = key.clone();
    // let dispatch = ctx.ctrl.dispatch.clone();
    // let begin_edit = move || (dispatch)(GridEvent::BeginEdit(key_for_closure));

    // let clone_is_active = is_active.clone();
    // let clone_set_active = set_active.clone();
    let set_active2 = set_active.clone();
    let begin_edit2 = begin_edit.clone();

    view! {
        <div
            id=dom_id
            class=move || format!("grid-cell {} {}", col.class, if is_active() { "is-active" } else { "" })
            role="gridcell"
            tabindex=tabindex
            data-column=col_id.0.to_string()
            on:focus=move |_| set_active()
            on:click=move |_| set_active2()
            on:dblclick=move |_| {
                if is_editable { begin_edit(); }
            }
            on:keydown=move |ev| {
                // Minimal key contract sketch:
                // Enter: begin edit, Esc: cancel edit (when editing)
                let k = ev.key();
                if k == "Enter" && is_editable {
                    ev.prevent_default();
                    begin_edit2();
                }
                if k == "Escape" {
                    dispatch(GridEvent::CancelEdit);
                }
                // Arrow key nav intentionally omitted; implement in GridRoot with activedescendant model.
            }
        >
            {move || {
                if is_editing() {
                    if let Some(editor) = &editor_fn {
                        return (editor)(&row);
                    }
                }
                (cell_fn)(&row)
            }}
        </div>
    }
}

// use leptos::prelude::*;
// use std::sync::Arc;
// use uuid::Uuid;

use crate::ipc;

// Your row type
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EntryRow {
    pub uuid: Uuid,
    pub name: String,
    pub note: String,
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

// --- Example page that uses the grid ---
#[component]
pub fn EntriesGridDemo() -> impl IntoView {
    // Kick off the async work (use LocalResource on wasm — futures aren't `Send`)
    let entries = LocalResource::new(|| async move {
        ipc::list_entry(EntryFilter::new().sort_by(Some(EntrySortField::default())))
            .await
            .map(|entries| {
                let mut result = entries
                    .into_iter()
                    .map(|entry| EntryRow::from(&entry))
                    .collect::<Vec<_>>();
                result.truncate(100);
                result
            })
    });

    // 4) Render
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
                    Some(Ok(v)) => {
                        let (rows, _) = signal(v);
                        view! { <EntriesGridDemoInner rows=rows /> }.into_any()
                    },
                }
            }}
        </Suspense>
        </ErrorBoundary>
    }
}

// --- Example page that uses the grid ---
#[component]
pub fn EntriesGridDemoInner(rows: ReadSignal<Vec<EntryRow>>) -> impl IntoView {
    // 2) row_id: stable identity for selection + keying
    let row_id: Arc<dyn Fn(&EntryRow) -> RowId + Send + Sync + 'static> =
        Arc::new(|r: &EntryRow| RowId(r.uuid));

    // 3) Schema: headers + cell renderers (+ optional editor renderers)
    let schema = GridSchema::<EntryRow> {
        column_order: vec![ColumnId("name"), ColumnId("note")],
        columns: vec![
            ColumnDef::<EntryRow> {
                id: ColumnId("name"),
                header: Arc::new(|| view! { <span>"Name"</span> }.into_any()),
                cell: Arc::new(|r: &EntryRow| view! { <span>{r.name.clone()}</span> }.into_any()),
                // Example editor: just a placeholder “editor” view for now
                editor: Some(Arc::new(|r: &EntryRow| {
                    view! {
                        <input
                            class="cell-editor"
                            prop:value=r.name.clone()
                            // real editing would dispatch a patch event + update your store
                        />
                    }
                    .into_any()
                })),
                sortable: true,
                class: "col-name",
                header_class: "col-name-header",
            },
            ColumnDef::<EntryRow> {
                id: ColumnId("note"),
                header: Arc::new(|| view! { <span>"Note"</span> }.into_any()),
                cell: Arc::new(|r: &EntryRow| view! { <span>{r.note.clone()}</span> }.into_any()),
                editor: None,
                sortable: false,
                class: "col-note",
                header_class: "col-note-header",
            },
        ],
    };

    // 4) Render
    view! {
        <GridRoot
            grid_id="entries-grid"
            aria_label="Entries"
            schema=schema
            rows=rows
            row_id=row_id
            use_activedescendant=true
        >
            <GridHeader _marker={std::marker::PhantomData::<EntryRow>} />
            <GridBody _marker={std::marker::PhantomData::<EntryRow>} />
        </GridRoot>
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

// GridRoot
//  GridHeader ( writes all header cells)
//  GridBody
//   For each row:
//    GridRow (writes all cells; needs some virtualazation support as 1000 nodes make the UI slow as hell)
