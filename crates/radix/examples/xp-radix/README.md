# README

## Purpose

This project demonstrates how Radix UI primitives work in isolation.

Each example depends only on Radix primitives. All styling is provided through CSS classes defined in a single, monolithic stylesheet. This serves two goals:

1. A reference for which styles must be supplied by consumers of the library.
2. A behavioral and visual baseline for validating the Leptos Radix port against the React implementation.

When this same CSS file is used with the Leptos Radix library, the application should be visually and behaviorally identical to the React implementation. This parity makes it straightforward to verify correctness of the ported implementation.

## Application Structure

The application has a simple layout:
- A navigation bar on the left
- A display panel on the right

The navigation bar links to individual primitive examples. Selecting a link renders the corresponding example in the display panel.

Each example demonstrates both the appearance and behavior of a primitive across a representative set of configuration options. Explanatory text documents available options and relevant accessibility features.

## Porting Goal

The application is intentionally simple so it can be re-implemented using the Leptos Radix library. Matching behavior and visuals against this reference implementation provides a concrete validation target for the port.