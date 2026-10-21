# simplog_lazy_lock

Natural-language reasoning with `LazyLock` declarations.

`axiom!` and `theorem!` declare public `LazyLock<Prop>` statics. Their statement argument accepts `impl Into<String>`, including expressions evaluated on first access. Dependency lists such as `[p, q]` store references to lazy propositions without forcing them. Dependency chains must be acyclic.

Construct a decision with `Value::new(inner, const { &[&p, &q] })`. The inner value can be constructed at runtime, including from local borrows. `Value<T>` implements `Deref<Target = T>` and `AsRef<T>`.

Repository, commit, and source declarations use the same macro names as `simplog_direct`. The `github_commit!` and `source_github!` macros accept the referenced declaration's path without `&`. Source metadata uses `GitOid` (an alias for `gix_hash::ObjectId`), `PathBuf`, and `Url`. The `github_commit!` macro accepts a hexadecimal string literal encoding a 20-byte SHA-1 object ID, validated at compile time with `hex_literal::hex!`, or a `GitOid` expression. The `source_html_page!` macro accepts a URL string literal and constructs its `Url` with `url_macro::url!`. Both validation macros are re-exported internally, so callers need only depend on Simplog.

An axiom's optional source argument accepts `impl Into<Option<Source>>`. For a source declared with `source_html_page!` or `source_github!`, pass `source.clone()`.

For explanations with explicit terminology, `definition!(name, term, meaning, [definitions], source)` declares a `Definition`. Its trailing definition list and source are optional. Definitions describe meanings; they are not proposition premises. Their links must also be acyclic.

An axiom can specify its role and vocabulary with `axiom!(name, statement, source, kind, [definitions])`, where `kind` is `AxiomKind::Fact`, `Assumption`, or `Requirement`. Older calls use `Unspecified`. The optional definition list defaults to empty.

Use `theorem!(name, statement, [premises], inference, [definitions])` to explain the inference separately from the conclusion. The optional inference text and definition list default to empty for older calls. Conditions needed by an inference belong in the statement or its explicit assumption premises. Source evidence, definitions, and inference dependencies occupy separate fields.

`question!(name, statement, [definitions], [context])` records an unresolved question. The trailing lists are optional. Context links identify relevant propositions without asserting an answer. Questions are not propositions and cannot be passed as theorem premises. A `Value` has separate `definitions` and `questions` slices, initially empty, so callers can attach explanations to a decision while keeping its `dependencies` limited to justification.

Build the proposition-only library examples and run the decision example from the workspace root:

```sh
cargo build -p simplog_lazy_lock --example socrates_lazy_lock
cargo build -p simplog_lazy_lock --example two_plus_two_lazy_lock
cargo run -p simplog_lazy_lock --example ll_vs_dc_lazy_lock
```

The matching direct-construction examples use `-p simplog_direct` and omit the `_lazy_lock` suffix. Distinct target names avoid Cargo output collisions when building the whole workspace. In both versions, `ll_vs_dc` stores its decision in `Value<Construction>`, whose variants are `LazyLock` and `Direct`. Both choose `Direct` under the example's explicit assumptions.
