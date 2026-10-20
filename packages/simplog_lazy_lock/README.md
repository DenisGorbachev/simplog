# simplog_lazy_lock

Natural-language reasoning with `LazyLock` declarations.

`axiom!` and `theorem!` declare public `LazyLock<Prop>` statics. Their content accepts `impl Into<String>`, including expressions evaluated on first access. Dependency lists such as `[p, q]` store references to lazy propositions without forcing them. Dependency chains must be acyclic.

Construct a decision with `Value::new(inner, const { &[&p, &q] })`. The inner value can be constructed at runtime, including from local borrows. `Value<T>` implements `Deref<Target = T>` and `AsRef<T>`.

Repository, commit, and source declarations use the same macro names as `simplog`. The `github_commit!` and `source_github!` macros accept the referenced declaration's path without `&`. Source metadata uses `GitOid`, `PathBuf`, and `Url`. The `source_html_page!` macro accepts a URL string literal and constructs its `Url` with `url_macro::url!`.

An axiom's optional source argument accepts `impl Into<Option<Source>>`. For a source declared with `source_html_page!` or `source_github!`, pass `source.clone()`.

Build the proposition-only library examples and run the decision example from the workspace root:

```sh
cargo build -p simplog_lazy_lock --example socrates_lazy_lock
cargo build -p simplog_lazy_lock --example two_plus_two_lazy_lock
cargo run -p simplog_lazy_lock --example ll_vs_dc_lazy_lock
```

The matching direct-construction examples use `-p simplog` and omit the `_lazy_lock` suffix. Distinct target names avoid Cargo output collisions when building the whole workspace. In both versions, `ll_vs_dc` stores its decision in `Value<Construction>`, whose variants are `LazyLock` and `Direct`. Both choose `Direct` under the example's explicit assumptions.
