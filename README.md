# ac-qu-ai-nt

`ac-qu-ai-nt` is a project I'm working on, using artificial intelligence to break down the user's query, acquire knowledge, and transfer insights to the user('s mind).

This is a placeholder while I take small steps to build out the project.

# Contributing

This is a summary of the crates I expect to be in this project and how they are used:

```mermaid
flowchart TD
core --> cli-clap
core --> gui-eframe
core --> tui-ratatui
cli-clap --> multibinary
gui-eframe --> multibinary
tui-ratatui --> multibinary
```

## Versioning

All the crates in this project have a [minimum supported Rust version (MSRV)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html) of 1.76, the release succeeding [the one that stabilized `async fn` in traits](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html) (which the `core` crate of this project is expected to make use of). I intend to test this in CI (GitHub Actions) in the future to identify if it ever gets raised by changes but I have not set this up yet. Raising the MSRV will be considered a breaking change --- my justification is that it [seems more appropriate for an application like this](https://github.com/matklad/once_cell/issues/201#issuecomment-1257213601). This may be reconsidered if [Cargo's MSRV-aware resolver](https://rust-lang.github.io/rfcs/3537-msrv-resolver.html) is stabilized.
