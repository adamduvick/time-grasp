## Radix Project Structure

- `crates/radix/examples/react-radix-primitives/` - React Radix source (reference implementation)
- `crates/radix/examples/ts-xp-radix/` - TypeScript/React examples (behavior baseline)
- `crates/radix/examples/xp-radix/` - Leptos examples (should match ts-xp-radix)

## Porting Workflow

1. Each example depends only on Radix primitives with styling via CSS classes
2. A single, monolithic stylesheet defines all styles
3. When the same CSS is used with Leptos Radix, the app should be visually and behaviorally identical
4. This parity validates correctness of the ported implementation

## Application Layout

- Navigation bar on the left links to individual primitive examples
- Display panel on the right renders the selected example
- Each example demonstrates appearance and behavior across representative configurations
- Explanatory text documents available options and accessibility features

---

## Primitive Dependency Hierarchy

When implementing or refactoring Leptos primitives, work bottom-up through these levels.

### LEVEL 0: FOUNDATIONAL (No Radix Dependencies)

Pure utilities - implement first:
- **compose-refs** - Ref composition utility
- **context** - React context helpers
- **direction** - RTL/LTR direction context
- **focus-guards** - Focus trap boundary guards
- **use-callback-ref** - Callback ref hook
- **use-layout-effect** - SSR-safe useLayoutEffect
- **use-previous** - Previous value hook
- **use-is-hydrated** - Hydration state hook
- **use-rect** - Element rect measurement

### LEVEL 1: BASIC BUILDING BLOCKS (1-2 foundational deps)

Simple hooks and utilities:
- **use-effect-event** - depends on: use-layout-effect
- **use-size** - depends on: use-layout-effect
- **id** - depends on: use-layout-effect
- **use-escape-keydown** - depends on: use-callback-ref
- **use-controllable-state** - depends on: use-effect-event, use-layout-effect

Component utilities:
- **slot** - depends on: compose-refs
- **primitive** - depends on: slot
- **presence** - depends on: compose-refs, use-layout-effect

### LEVEL 2: INTERMEDIATE BUILDING BLOCKS

Simple component wrappers:
- **arrow** - depends on: primitive
- **aspect-ratio** - depends on: primitive
- **label** - depends on: primitive
- **separator** - depends on: primitive
- **visually-hidden** - depends on: primitive
- **progress** - depends on: context, primitive

Advanced utilities:
- **collection** - depends on: compose-refs, context, primitive, slot
- **portal** - depends on: primitive, use-layout-effect
- **focus-scope** - depends on: compose-refs, primitive, use-callback-ref
- **dismissable-layer** - depends on: primitive, compose-refs, use-callback-ref, use-escape-keydown

### LEVEL 3: COMPLEX BUILDING BLOCKS

- **popper** - depends on: arrow, compose-refs, context, primitive, use-callback-ref, use-layout-effect, use-rect, use-size (+ @floating-ui/react-dom)
- **roving-focus** - depends on: primitive, collection, compose-refs, context, direction, id, use-callback-ref, use-controllable-state

### LEVEL 4: USER-FACING PRIMITIVES - SIMPLE

Standalone controls (no other user-facing deps):
- **avatar** - depends on: context, primitive, use-callback-ref, use-is-hydrated, use-layout-effect
- **checkbox** - depends on: primitive, compose-refs, context, presence, use-controllable-state, use-previous, use-size
- **collapsible** - depends on: primitive, compose-refs, context, id, presence, use-controllable-state, use-layout-effect
- **switch** - depends on: primitive, compose-refs, context, use-controllable-state, use-previous, use-size
- **toggle** - depends on: primitive, use-controllable-state
- **tabs** - depends on: primitive, context, direction, id, presence, roving-focus, use-controllable-state
- **radio-group** - depends on: primitive, compose-refs, context, direction, presence, roving-focus, use-controllable-state, use-previous, use-size
- **slider** - depends on: primitive, collection, compose-refs, context, direction, use-controllable-state, use-layout-effect, use-previous, use-size
- **scroll-area** - depends on: primitive, compose-refs, context, direction, presence, use-callback-ref, use-layout-effect

### LEVEL 5: USER-FACING PRIMITIVES - OVERLAY/PORTAL BASED

- **dialog** - depends on: primitive, compose-refs, context, dismissable-layer, focus-guards, focus-scope, id, portal, presence, slot, use-controllable-state (+ aria-hidden, react-remove-scroll)
- **popover** - depends on: primitive, compose-refs, context, dismissable-layer, focus-guards, focus-scope, id, popper, portal, presence, slot, use-controllable-state (+ aria-hidden, react-remove-scroll)
- **hover-card** - depends on: primitive, compose-refs, context, dismissable-layer, popper, portal, presence, use-controllable-state
- **tooltip** - depends on: primitive, compose-refs, context, dismissable-layer, id, popper, portal, presence, slot, use-controllable-state, visually-hidden
- **toast** - depends on: primitive, collection, compose-refs, context, dismissable-layer, portal, presence, use-callback-ref, use-controllable-state, use-layout-effect, visually-hidden

### LEVEL 6: USER-FACING PRIMITIVES - MENU BASED

Menu system (shares common menu primitive):
- **menu** - depends on: primitive, collection, compose-refs, context, direction, dismissable-layer, focus-guards, focus-scope, id, popper, portal, presence, roving-focus, slot, use-callback-ref (+ aria-hidden, react-remove-scroll)
- **dropdown-menu** - depends on: primitive, compose-refs, context, id, **menu**, use-controllable-state
- **context-menu** - depends on: primitive, context, **menu**, use-callback-ref, use-controllable-state
- **menubar** - depends on: primitive, collection, compose-refs, context, direction, id, **menu**, roving-focus, use-controllable-state

### LEVEL 7: USER-FACING PRIMITIVES - COMPLEX COMPOSITES

Components built on other user-facing primitives:
- **alert-dialog** - depends on: primitive, compose-refs, context, **dialog**, slot
- **accordion** - depends on: primitive, **collapsible**, collection, compose-refs, context, direction, id, use-controllable-state
- **toggle-group** - depends on: primitive, context, direction, roving-focus, **toggle**, use-controllable-state
- **toolbar** - depends on: primitive, context, direction, roving-focus, **separator**, **toggle-group**
- **select** - depends on: primitive, collection, compose-refs, context, direction, dismissable-layer, focus-guards, focus-scope, id, popper, portal, slot, use-callback-ref, use-controllable-state, use-layout-effect, use-previous, visually-hidden (+ aria-hidden, react-remove-scroll)
- **navigation-menu** - depends on: primitive, collection, compose-refs, context, direction, dismissable-layer, id, presence, use-callback-ref, use-controllable-state, use-layout-effect, use-previous, visually-hidden

### LEVEL 8: SPECIALIZED FORM FIELDS

- **form** - depends on: primitive, compose-refs, context, id, **label**
- **password-toggle-field** - depends on: primitive, compose-refs, context, id, use-controllable-state, use-effect-event, use-is-hydrated
- **one-time-password-field** - depends on: primitive, collection, compose-refs, context, direction, roving-focus, use-controllable-state, use-effect-event, use-is-hydrated, use-layout-effect

---

## Key Dependency Patterns

### Most Depended-Upon Utilities
- **primitive** - Used by almost all components
- **compose-refs** - Used extensively for ref management
- **context** - Used for component state sharing
- **use-controllable-state** - Used by all interactive components
- **use-layout-effect** - Foundational hook used widely

### Positioning Stack
```
arrow → popper → (hover-card, tooltip, menu, popover, select)
```

### Focus Management Stack
```
use-callback-ref → focus-scope + dismissable-layer → (dialog, popover, menu, select)
```

### Menu Hierarchy
```
menu (base) → dropdown-menu, context-menu, menubar (variants)
```

### Portal-Based Overlays
- All use: portal + presence + dismissable-layer
- Dialog family: dialog → alert-dialog
- Positioning overlays use popper: tooltip, hover-card, popover, menu

---

## Leptos Implementation Notes

### Primitives to Prioritize for Refactoring
1. **popper** - Foundation for all positioned content
2. **menu** - Foundation for dropdown-menu, context-menu, menubar
3. **dialog** - Foundation for alert-dialog
4. **collapsible** - Foundation for accordion
5. **toggle** - Foundation for toggle-group
6. **separator** - Foundation for toolbar

### Current Leptos Implementation Status
Check `crates/radix/examples/xp-radix/src/primitives.json` for `implemented: true/false` flags.
