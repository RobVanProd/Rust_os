# Contributing to the Rust-Native GUI Operating System

We welcome contributions to help build this exciting new operating system! Whether you're interested in kernel development, UI design, application building, or documentation, there's a place for you.

## Core Principles
Before contributing, please familiarize yourself with the project's [Vision & Core Principles](docs/00-vision-core-principles/README.md) and the overall [Development Roadmap](docs/ROADMAP.md).

## Getting Started
1.  **Set up your Development Environment**: Refer to the [Developer SDK & Community Ecosystem](docs/12-developer-sdk-community-ecosystem/README.md) documentation for details on the Cargo target, QEMU simulation, and IDE setup.
2.  **Find an Issue**: Look for open issues on our issue tracker (link to be added). Good first issues will be tagged appropriately.
3.  **Discuss your Plans**: For larger changes or new features, please open an issue or start a discussion thread to outline your proposal. This helps ensure your work aligns with the project's direction.

## Development Process

### RFC Process
For substantial changes to the OS architecture, core APIs, or fundamental components, we use an RFC (Request for Comments) process.
-   Proposals should be submitted as a pull request to an `rfcs` directory (to be created).
-   The RFC should detail the motivation, design, drawbacks, alternatives, and unresolved questions.
-   RFCs will be discussed by the community and core contributors before acceptance or rejection.

### Coding Style & Linting
-   Code should be formatted using `rustfmt` with the default project settings.
-   We use `clippy` for linting. Please address all `clippy` warnings before submitting code.
-   Strive for clear, well-commented, and memory-safe Rust code. `unsafe` blocks should be minimized, heavily justified, and reviewed.

### Testing
-   New features should be accompanied by unit tests.
-   Integration tests should be added for larger components.
-   Our CI (Continuous Integration) pipeline, likely using `cargo nextest` in an emulated environment (QEMU), will run tests automatically. All tests must pass for a PR to be merged. (Details on CI setup will be added to [Developer SDK & Community Ecosystem](docs/12-developer-sdk-community-ecosystem/README.md)).

### Commit Messages
Please follow conventional commit message guidelines:
-   A short, imperative summary line (e.g., "Add feature X for Y module").
-   A more detailed explanatory text, if necessary, separated by a blank line.

### Pull Requests
-   Fork the repository and create a new branch for your feature or bug fix.
-   Ensure your branch is up-to-date with the main branch before submitting a pull request.
-   Provide a clear description of the changes in your pull request.
-   Link any relevant issues.
-   Ensure all CI checks pass.

## Community
Join our community platforms (links to be added in [Developer SDK & Community Ecosystem](docs/12-developer-sdk-community-ecosystem/README.md)) to ask questions, share ideas, and collaborate with other developers.

We look forward to your contributions!
