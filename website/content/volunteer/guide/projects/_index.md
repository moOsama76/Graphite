+++
title = "Projects"
template = "book.html"
page_template = "book.html"

[extra]
order = 3 # Chapter number
+++

Graphite is built from a number of separate projects, each with a distinct focus. New developers may choose to specialize in one or more area without having to attain a working knowledge of the entire codebase. This chapter details each project's purpose, what knowledge or background is best suited for getting involved with it, and how to begin making useful contributions. Opportunities also exist for contributing to specific self-contained sub-projects for students interested in completing a Google Summer of Code or other internship program, a university capstone project, or another similar endeavor.

## Frontend

The frontend is the interface for Graphite which users see and interact with. It is built using web technologies with TypeScript, Svelte, and SCSS. The frontend's philosophy is to be as lightweight and minimal as possible. It acts as the entry point for user input and quickly hands off its work to the WebAssembly editor backend via its Wasm wrapper API. That API is written in Rust but has TypeScript bindings generated by the wasm-bindgen tooling that is part of the Vite-based build chain. The frontend is built of many components that recursively form the window, panels, and widgets that make up the user interface.

## Editor

The editor is the core of the Graphite application, and it's where all the business logic occurs for the tooling and user interaction. It is written in Rust and compiled to WebAssembly, and at its heart is the message system [described here](../codebase-overview/code-structure#the-message-system).

## Graphene

Graphene is the node graph engine which manages and renders the documents. It is itself a programming language, where Graphene programs are compiled while being edited live by the user, and where executing the program renders the document.

## Libraries

Graphite maintains several Rust libraries that are published to crates.io for use by other developers, in addition to internal use by Graphite itself.

### Bezier-rs

Bezier-rs is a computational geometry library. It provides a rich selection of types and functions for working with Bézier segments and paths. There is also an [interactive demo](/libraries/bezier-rs/) gallery. This was initially developed as a successful [student project](./student-projects). Contribution opportunities include optimization, code cleanup and refactoring, and implementing new algorithms.