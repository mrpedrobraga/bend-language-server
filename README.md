# Bend Language Server

Implementation of the Language Server Protocol (using [tower-lsp](https://crates.io/crates/tower-lsp)) for the Bend Programming Language.

This uses [bend-lang](https://github.com/HigherOrderCO/Bend), the official Bend compiler as a library and exposes functionality through the LSP, to provide diagnostics, hover information, etc.

## Contributing

Here is a rough to-do list of things to do (non-exhaustive, always increasing).

- [ ] Code Actions for building Releases. Releases are important, as editor extensions like [Zed's](https://github.com/mrpedrobraga/zed-bend) to download an up-to-date binary.
- [ ] Diagnostics
- [ ] Hover Information
- [ ] Code Actions
