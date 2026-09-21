# dez Extensions

This directory contains extensions for dez that are largely maintained by the dez team. They currently live in the dez repository for ease of maintenance.

If you are looking for the dez extension registry, see the [`zed-industries/extensions`](https://github.com/zed-industries/extensions) repo.

## Structure

Currently, dez includes support for a number of languages without requiring installing an extension. Those languages can be found under [`crates/languages/src`](https://github.com/zed-industries/zed/tree/main/crates/languages/src).

Support for all other languages is done via extensions. This directory ([extensions/](https://github.com/zed-industries/zed/tree/main/extensions/)) contains some of the officially maintained extensions. These extensions use the same [dez_extension_api](https://docs.rs/dez_extension_api/latest/dez_extension_api/) available to all [dez Extensions](https://dez.dev/extensions) for providing [language servers](https://dez.dev/docs/extensions/languages#language-servers), [tree-sitter grammars](https://dez.dev/docs/extensions/languages#grammar) and [tree-sitter queries](https://dez.dev/docs/extensions/languages#tree-sitter-queries).

You can find the other officially maintained extensions in the [dez-extensions organization](https://github.com/dez-extensions).

## Dev Extensions

See the docs for [Developing an Extension Locally](https://dez.dev/docs/extensions/developing-extensions#developing-an-extension-locally) for how to work with one of these extensions.
