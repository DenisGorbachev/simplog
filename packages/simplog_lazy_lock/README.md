# simplog_lazy_lock

Natural-language reasoning with `LazyLock` declarations.

`axiom!` and `theorem!` declare public `LazyLock<Prop>` statics. Their content accepts `impl Into<String>`, including expressions evaluated on first access. Dependency lists such as `[P, Q]` store references to lazy propositions without forcing them. Dependency chains must be acyclic.

`value!(inner, [P, Q])` returns an ordinary `Value<T>`. The inner value can be constructed at runtime, including from local borrows. `Value<T>` implements `Deref<Target = T>` and `AsRef<T>`.

Repository, commit, and source declarations use the same macro names as `simplog`. The `github_commit!`, `github_source!`, and `source_github!` macros accept the referenced declaration's path without `&`. Constructors accept references to the corresponding `LazyLock` statics. Source metadata uses `GitOid`, `PathBuf`, and `Url`; callers supply already parsed identifiers and URLs.

An axiom's optional source argument is an `Option<Source>`. For a source declared with `source_html_page!` or `source_github!`, pass `Some(SOURCE.clone())`.

Run the examples from the workspace root:

```sh
cargo run -p simplog_lazy_lock --example socrates
cargo run -p simplog_lazy_lock --example two_plus_two
cargo run -p simplog_lazy_lock --example ll_vs_dc
```

The matching direct-construction examples use `-p simplog`. In both versions, `ll_vs_dc` stores its decision in `Value<bool>`: `true` selects LL and `false` selects DC. Both choose DC under the example's explicit assumptions; the LazyLock version demonstrates expressing the same reasoning with lazy declarations.
