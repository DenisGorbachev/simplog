# simplog repo spec

- Every Rust source file:
  - May have the following declarations:
    - `use derive_more::{Deref, AsRef};`

## package simplog

- Every struct except [`Value`](#struct-value) and envelope payload types:
  - Must have a [declaration macro](#macro-declaration-of-t-requirements).
- Every envelope payload type:
  - Must not have a standalone declaration macro.
- Every [envelope](#envelope-enum):
  - Every variant:
    - Must have a [declaration macro](#macro-declaration-of-enum-e-of-variant-v-requirements).
- Every specified macro:
  - Must use `macro_rules!`.
  - Must have `#[macro_export]`.
  - Must accept optional trailing commas only in variadic argument lists.
- Every type whose fields implement `Copy`:
  - Must implement `Copy`.
  - Must implement `Clone`.
- Every macro accepting `dependencies`:
  - Must accept a list of proposition paths, e.g. `[p, q]`.
  - Must store a static slice of references, e.g. `&[&p, &q]`.

### struct GithubRepo

- Must have fields:
  - `owner: &'static str`
  - `name: &'static str`

### struct GithubCommit

- Must have fields:
  - `repo: &'static GithubRepo`
  - `oid: &'static str`

### struct GithubSource

- Must have fields:
  - `commit: &'static GithubCommit`
  - `path: &'static str`
    - /// Caller guarantees a nonempty repository-relative path without `..` components, identifying a file in `commit`.
  - `line_range: (usize, usize)`
    - /// Inclusive, one-based bounds. Caller guarantees `1 <= start <= end <= line_count` for the file in `commit`.
- The `source_github!` macro:
  - Must accept a tuple of endpoints.
  - Must normalize their order.

### struct HtmlPageSource

- Must have fields:
  - `url: &'static str`

### enum Source

- Must have variants:
  - `Github(GithubSource)`
  - `HtmlPage(HtmlPageSource)`

### struct AxiomProp

- Must have fields:
  - `content: &'static str`
  - `source: Option<Source>` (optional)

### struct TheoremProp

- Must have fields:
  - `content: &'static str`
  - `dependencies: &'static [&'static Prop]`

### enum Prop

- Must have variants:
  - `Axiom(AxiomProp)`
  - `Theorem(TheoremProp)`

### struct Value

- Must have generics:
  - `T`
- Must have derives:
  - `Deref`
  - `AsRef`
- Must have fields:
  - `inner: T`
  - `dependencies: &'static [&'static Prop]`
- Must not have a constructor macro.

### Macro declaration of T requirements

- Must be named `{snake_case(T)}`.
- Must accept the declaration identifier before field values.
- Must accept field values in field order.
- Must accept generic arguments for generic types.
- Must have one arm for every argument list obtained by omitting zero or more trailing optional fields.
- Every arm:
  - Must emit a complete `pub static name: T = ...;` declaration.
  - Must include the type's generic arguments.
  - Must initialize the static with a struct literal.
  - Must use const defaults for omitted fields.
  - Must use `$crate` paths for simplog items.
  - Must evaluate each supplied argument once.

### Macro declaration of enum E of variant V requirements

- Must follow [T declaration requirements](#macro-declaration-of-t-requirements), except as specified below.
- Must be named `{snake_case(E)}_{snake_case(V)}` when `E` is not `Prop`.
- Must be named `{snake_case(V)}` when `E` is `Prop`.
- Must accept the inner type's field values.
- Must support the inner type's optional argument lists.
- Every arm:
  - Must emit `pub static name: E = E::V(...);`.
  - Must construct the inner value with a struct literal.

### Macro constructor of T requirements

- Must be named `{snake_case(T)}`.
- Must have one arm for every argument list obtained by omitting zero or more trailing optional fields.
- Every arm:
  - Must call a `T` constructor.
  - Must call `.clone()` on concrete, non-generic, non-`Copy` arguments that are cheaply clonable (e.g. `Arc`, `Rc`).

### Macro constructor of Arc of T requirements

- Must be named `arc_{snake_case(T)}`.
- Must have the same arms as the [T constructor macro](#macro-constructor-of-t-requirements).
- Every arm:
  - Must call the T constructor macro.
  - Must wrap its result in `Arc::new`.

### Macro constructor of enum E of variant V requirements

- Must be named `{snake_case(E)}_{snake_case(V)}`.
- Must have the same arms as the [T constructor macro](#macro-constructor-of-t-requirements), where `T` is the variant's inner type.
- Every arm:
  - Must call the T constructor macro.
  - Must wrap its result in `E::V`.

### Macro constructor of Arc of enum E of variant V requirements

- Must be named `arc_{snake_case(E)}_{snake_case(V)}`.
- Must have the same arms as the [variant constructor macro](#macro-constructor-of-enum-e-of-variant-v-requirements).
- Every arm:
  - Must call the variant constructor macro.
  - Must wrap its result in `Arc::new`.

### Envelope enum

An enum whose variants each contain exactly one tuple field.

## package simplog_lazy_lock

- Must follow the `simplog` requirements with the following changes.
- Every struct:
  - Must derive `new` from `derive-new`.
  - Must use `#[new(into)]` on `String`, `PathBuf`, and `Option` fields.
- Every declaration macro:
  - Must declare a `LazyLock<T>` static.
  - Must construct the value inside its initializer.
  - Must call the struct or envelope payload's derived constructor instead of using a struct literal.
- Every string field except `GithubCommit::oid`:
  - Must use an owned type: `String` for text, `PathBuf` for paths, `Url` for URLs.
- Every reference to `GithubRepo`, `GithubCommit`, or `Prop`:
  - Must reference `LazyLock<T>`.
- `GithubCommit::oid`:
  - Must use `GitOid`, an alias for `git2::Oid`.
- The `github_commit!` and `source_github!` macros:
  - Must accept the referenced declaration's path without `&`.
- The `source_html_page!` macro:
  - Must accept the string-literal argument accepted by `url_macro::url!`.
  - Must construct the URL with `url_macro::url!`.
