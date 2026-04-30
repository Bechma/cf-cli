# CyberFabric CLI Design Proposal

This folder contains a design-only proposal for extending `cargo cyberfabric`
around project templates, a CLI-owned manifest, discovery, help, linting, tests,
development runs, and builds. It is based on the current implementation in this
repository and intentionally does not change Rust code.

# Motivation

## Why we need this CLI tool for CyberFabric

- ...

## What this tool do

- Help on the project scaffolding
- It will help on the lifecycle software development
- ...

## What this tool does NOT do

...

## What this tool will not do

...

## Scope of the current tooling in Rust

### Cargo

- Creates simple hello world main.rs or lib.rs
- ...

## Table of Contents

1. [Manifest and Configuration](./01-manifest-and-config.md)
2. [Template Generation](./02-template-generation.md)
3. [List and Inspection](./03-list-and-inspection.md)
4. [Help and Docs](./04-help-and-docs.md)
5. [Lint](./05-lint.md)
6. [Test](./06-test.md)
7. [Run for Development](./07-run-dev.md)
8. [Build and Package](./08-build-and-package.md)
9. [Implementation Plan](./09-implementation-plan.md)

## Design Goals

- Move application composition out of runtime configuration and into a
  CLI-owned manifest.
- Keep runtime configuration focused on values consumed by the generated server
  and modules.
- Keep command behavior predictable for both humans and LLM-driven automation.
- Preserve the current CLI flow where possible: `init`, `mod add`,
  `config mod ...`, `docs`, `lint`, `run`, `build`, and `deploy`.
- Prefer typed command modes and schema enums over open string values when the
  allowed values are known.
- Make generated artifacts explicit enough that users can inspect, reproduce,
  and override them.

## Proposed Command Shape

```text
cargo cyberfabric
|-- new/init
|-- generate
|   |-- workspace
|   |-- module
|   |-- config
|   |-- manifest
|   |-- build
|   |-- agents
|   `-- skill
|-- list
|   |-- modules
|   |-- system-modules
|   |-- local-modules
|   `-- configs
|-- help
|   |-- schema
|   |-- docs
|   `-- topic
|-- manifest
|   |-- add
|   |-- edit
|   |-- rm
|   |-- validate
|   `-- render
|-- config
|   |-- db
|   `-- mod
|-- lint
|-- test
|-- run
|-- build
`-- deploy
```

The tree keeps the existing command names where they already work, while adding
more explicit aliases and subcommands for the new manifest-first model.

## Key Decision

The manifest becomes the source of truth for what the CLI generates and
orchestrates. Runtime config remains the source of truth for runtime settings.

In practical terms:

- Manifest answers: which app, environment, modules, feature sets, test
  strategy, lint policy, runner mode, and build outputs should the CLI use?
- Runtime config answers: what values should the generated server and modules
  read at runtime?

