# CyberFabric CLI Design Proposal

This folder contains a design proposal for a CLI tool for CyberFabric.

# Motivation

In CyberFabric we are developing a new framework for building modular applications.
This approach allows us to design backend applications in a more structured way, however, it comes with some challenges.

There's a high cognitive complexity for any new user of the framework, regardless of his background.
This CLI tool pretends to be an opinionated but yet flexible tool to help users to build their applications.
For most of the application life cycle, from scaffolding a simple app, to the orchestration of a bunch of applications.

We are not reinventing the wheel with this, specifically in Rust, we have a good ecosystem of tooling that helps the
developer during app development. The idea is to orchestrate the existing tooling, NOT replacing it.

By using this tool, the developer can focus on writing modules and the business logic that it's the relevant part.
Cyberfabric libraries(Modkit*) will leverage the framework runtime, system modules will provide generic functionality
to the application and Cyberfabric CLI(this tool) will provide a way to orchestrate all development tooling.

## Table of Contents

1. [Manifest and Configuration](./v1/01-manifest-and-config.md)
2. [Template Generation](./v1/02-template-generation.md)
3. [List and Inspection](./v1/03-list-and-inspection.md)
4. [Help and Docs](./v1/04-help-and-docs.md)
5. [Lint](./v1/05-lint.md)
6. [Test](./v1/06-test.md)
7. [Run for Development](./v1/07-run-dev.md)
8. [Build and Package](./v1/08-build-and-package.md)
9. [Implementation Plan](./v1/09-implementation-plan.md)

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

