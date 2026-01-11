# Frontend

Everything should be WAI-ARIA compliant. This is important for semantics, keyboard navigation, and for aiding users who have visual impairments.

Entry Grid:

[ARIA Grid](https://www.w3.org/WAI/ARIA/apg/patterns/grid/)

ARIA roles:

- `grid`
  - class: grid-container grid-container--<entity-name>
- `rowgroup`
  - class: grid-row-group grid-row-group--<group-name>
  - notes: *Header and Body groups*
- `row`
  - class: grid-row
- `columnheader`
  - class: grid-column-header grid-cell column--<column-name>
- `rowheader`
  - class: grid-row-header grid-cell row--<row-name>
  - note: *not needed for use cases*
- `gridcell`
  - class: grid-cell column--<column-name>
  - note: *would also include row-name class if rows were named*


```rust
#[component]
fn Grid(columns: Vec<Column>, rows: Vec<Row>) -> impl IntoView;

#[component]
fn GridHeader(columns: Vec<Column>) -> impl IntoView;

#[component]
fn GridBody(columns: Vec<Column>, rows: Vec<Row>) -> impl IntoView;

#[component]
fn GridBodyRow(columns: Vec<Column>, row: Row) -> impl IntoView;

trait Grid {
    type GridColumn: Column;
    type GridRow: Row;

    fn columns(&self) -> Vec<self::GridColumn>;
    fn rows(&self) -> Vec<self::GridRow>;
}

trait Column {
    fn name(&self) -> &'static str;
}

```