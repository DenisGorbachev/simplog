# simplog_lazy_lock

Natural-language reasoning with `LazyLock` declarations.

`axiom!` and `theorem!` declare public `LazyLock<Prop>` statics. Their statement argument accepts `impl Into<String>`, including expressions evaluated on first access. Dependency lists such as `[p, q]` store references to lazy propositions without forcing them. Dependency chains must be acyclic.

Construct a decision with `Value::new(inner, const { &[&p, &q] })`. The inner value can be constructed at runtime, including from local borrows. `Value<T>` implements `Deref<Target = T>` and `AsRef<T>`.

Repository, commit, and source declarations use the same macro names as `simplog_direct`. The `github_commit!` and `source_github!` macros accept the referenced declaration's path without `&`. Source metadata uses `GitOid` (an alias for `gix_hash::ObjectId`), `PathBuf`, and `Url`. The `github_commit!` macro accepts a hexadecimal string literal encoding a 20-byte SHA-1 object ID, validated at compile time with `hex_literal::hex!`, or a `GitOid` expression. The `source_html_page!` macro accepts a URL string literal and constructs its `Url` with `url_macro::url!`. Both validation macros are re-exported internally, so callers need only depend on Simplog.

An axiom's optional source argument accepts `impl Into<Option<Source>>`. For a source declared with `source_html_page!` or `source_github!`, pass `source.clone()`.

For explanations with explicit terminology, `definition!(name, term, meaning, [definitions], source)` declares a `Definition`. Its trailing definition list and source are optional. Definitions describe meanings; they are not proposition premises. Their links must also be acyclic.

An axiom can link its vocabulary with `axiom!(name, statement, source, [definitions])`. The optional definition list defaults to empty. An axiom is an established claim stated without a derivation from other propositions. Being an axiom describes its role in the theory; a separate fact/assumption label adds no logical support. Unverified claims must not be admitted as axioms.

Use `theorem!(name, statement, [premises], [definitions])` for a conclusion that follows obviously from its direct premises. The optional definition list defaults to empty. If the deduction needs explanation, introduce intermediate propositions until each step is obvious. There is no separate inference text. Source evidence, definitions, and proposition dependencies occupy separate fields.

Questions record requests for missing information rather than established claims, so investigation tracking stays outside the theory API. A verified statement that information is absent can still be a proposition. A `Value` has a separate `definitions` slice, initially empty, while its `dependencies` contain its justification.

A conditional proposition such as "If P, then Q" asserts the implication without asserting P. Conditions whose applicability has not been established must remain inside conditional statements; they must not become standalone premises asserting that P holds.

Build the proposition-only library examples and run the decision example from the workspace root:

```sh
cargo build -p simplog_lazy_lock --example socrates_lazy_lock
cargo build -p simplog_lazy_lock --example two_plus_two_lazy_lock
cargo run -p simplog_lazy_lock --example ll_vs_dc_lazy_lock
```

The matching direct-construction examples use `-p simplog_direct` and omit the `_lazy_lock` suffix. Distinct target names avoid Cargo output collisions when building the whole workspace. In both versions, `ll_vs_dc` stores its decision in `Value<Construction>`, whose variants are `LazyLock` and `Direct`. Both choose `Direct` under the example's explicit assumptions.
