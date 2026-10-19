# simplog repo spec

- Every Rust source file:
  - May have the following declarations:
    - `use nu_path::RelativePathBuf;`
    - `use interval_zoo::IntervalStrictFiniteIncInc;`

## package simplog

- Every struct:
  - Must have a corresponding `macro_rules!` that passes [Macro declaration of T requirements](#macro-declaration-of-t-requirements)
- Every [envelope](#envelope-enum):
  - For each variant:
    - Must have a corresponding `macro_rules!` that passes [Macro declaration of enum E of variant V requirements](#macro-declaration-of-enum-e-of-variant-v-requirements)
- Model values must support direct construction in immutable static initializers.
  - Constructors used by declaration macros must be `const fn`.
  - Types whose fields implement `Copy` must implement `Copy` and `Clone`.

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
    - /// Caller must guarantee that `path` is non-empty, is relative to the repository root, contains no `..` components, and identifies a file in `commit`.
  - `line_range: (usize, usize)`
    - /// Inclusive, one-based line bounds. Caller must guarantee `1 <= start <= end <= line_count` for the file in `commit`.
- Its constructor must accept a tuple of endpoints and normalize their order.

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
  - `dependencies: &'static [&'static Prop]` (optional)

### enum Prop

- Must have variants:
  - `Axiom(AxiomProp)`
  - `Theorem(TheoremProp)`

### struct Value

- Must have generics:
  - `T`
- Must have fields:
  - `inner: T`
  - `dependencies: &'static [&'static Prop]` (optional)

### Macro declaration of T requirements

- Must have `#[macro_export]`.
- Must have a name equal to `{snake_case(T)}`.
- Must accept the declaration identifier followed by constructor arguments in field order.
  - For generic types, must also accept their generic arguments, e.g. `value!(COUNT: usize, 3)`.
- Must have one arm per prefix of fields that includes all required fields.
- Every arm:
  - Must emit a complete `pub static NAME: T = ...;` declaration, including the concrete type and its generic arguments.
  - Must initialize the static by calling a const constructor of T.
  - Must use const expressions for omitted fields' default values, e.g. `None` and `&[]`.
  - Must evaluate each supplied argument expression exactly once.
  - Must use `$crate` paths for simplog items.
- Examples:
  - `github_repo!(REPO, "owner", "name");`
  - `github_commit!(COMMIT, &REPO, "0123456789abcdef0123456789abcdef01234567");`
  - `github_source!(FILE, &COMMIT, "src/lib.rs", (10, 20));`
  - `value!(COUNT: usize, 3);`

### Macro declaration of enum E of variant V requirements

- Must have `#[macro_export]`.
- Must have a name equal to `{snake_case(E)}_{snake_case(V)}`, except for `Prop`, whose macros must be named `{snake_case(V)}`.
- Must accept the declaration identifier followed by the inner type's constructor arguments.
- Must support the same optional-field prefixes as the inner type's declaration macro.
- Every arm:
  - Must emit a complete `pub static NAME: E = E::V(...);` declaration, including the concrete type.
  - Must construct the variant's inner value using its const constructor.
  - Must use const expressions for omitted fields' default values.
  - Must evaluate each supplied argument expression exactly once.
  - Must use `$crate` paths for simplog items.
- Examples:
  - `source_html_page!(PAGE, "https://example.com");`
  - `axiom!(PREMISE, "P", Some(PAGE));`
  - `axiom!(ASSUMPTION, "Q");`
  - `theorem!(CONCLUSION, "P or Q", &[&PREMISE]);`

### Legacy macro constructors

Reference only; implementations are not required, and these rules do not apply to declaration macros.

### Macro constructor of T requirements

- Must have `#[macro_export]`
- Must have a name equal to `{snake_case(T)}`
- Must contain one arm per combination of [prefix list] of fields of T which contains at least all required fields
  - For example:
    - If a struct contains fields `a`, `b`, `c` (optional), `d` (optional), then its macro constructor must contain three arms:
      - One arm for fields: `a`, `b`
      - One arm for fields: `a`, `b`, `c`
      - One arm for fields: `a`, `b`, `c`, `d`
- Every arm:
  - Must call a `T` constructor
  - Must call `.clone()` on those constructor args that have a concrete non-generic type that is cheaply clonable (e.g. `Arc`, `Rc`)

### Macro constructor of Arc of T requirements

- Must have `#[macro_export]`
- Must have a name equal to `arc_{snake_case(T)}`
- Must have the same arms as the corresponding [macro constructor of T](#macro-constructor-of-t-requirements)
- Every arm:
  - Must call `Arc::new`
  - Must call a [macro constructor of T](#macro-constructor-of-t-requirements)

### Macro constructor of enum E of variant V requirements

- Must have `#[macro_export]`
- Must have a name equal to `{snake_case(E)}_{snake_case(V)}`
- Must have the same arms as the corresponding [macro constructor of T](#macro-constructor-of-t-requirements)
- Every arm:
  - Must call `E::V`
  - Must call a [macro constructor of T](#macro-constructor-of-t-requirements) (where `T` is the inner type of variant `V`)

### Macro constructor of Arc of enum E of variant V requirements

- Must have `#[macro_export]`
- Must have a name equal to `arc_{snake_case(E)}_{snake_case(V)}`
- Must have the same arms as the corresponding [Macro constructor of enum E of variant V requirements](#macro-constructor-of-enum-e-of-variant-v-requirements)
- Every arm:
  - Must call `Arc::new`
  - Must call a [Macro constructor of enum E of variant V requirements](#macro-constructor-of-enum-e-of-variant-v-requirements)

### Envelope enum

A enum whose every variant is a tuple with exactly one inner type.
