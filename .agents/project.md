# simplog repo spec

- Every Rust source file:
  - May have the following declarations:
    - `use nu_path::RelativePathBuf;`
    - `use interval_zoo::IntervalStrictFiniteIncInc;`

## package simplog

- Every struct:
  - Must have a corresponding `macro_rules!` that passes [Macro constructor of T requirements](#macro-constructor-of-t-requirements)
  - Must have a corresponding `macro_rules!` that passes [Macro constructor of Arc of T requirements](#macro-constructor-of-arc-of-t-requirements)
- Every [envelope](#envelope-enum):
  - For each variant:
    - Must have a corresponding `macro_rules!` that passes [Macro constructor of enum E of variant V requirements](#macro-constructor-of-enum-e-of-variant-v-requirements)

### struct GithubRepo

- Must have fields:
  - `owner: String`
  - `name: String`

### struct GithubCommit

- Must have fields:
  - `repo: Arc<GithubRepo>`
  - `oid: GitOid`

### struct GithubSource

- Must have fields:
  - `commit: Arc<GithubCommit>`
  - `path: PathBuf`
    - /// Caller must guarantee that `path` is non-empty, is relative to the repository root, contains no `..` components, and identifies a file in `commit`.
  - `line_range: RangeInclusive<NonZeroU64>`

### struct HtmlPageSource

- Must have fields:
  - `url: Url`

### enum Source

- Must have variants:
  - `Github(GithubSource)`
  - `HtmlPage(HtmlPageSource)`

### struct ExcerptProp

- Must have fields:
  - `content: String`
  - `source: Source`

### struct InferenceProp

- Must have fields:
  - `content: String`
  - `dependencies: Vec<Arc<Prop>>` (optional)

### enum Prop

- Must have variants:
  - `Excerpt(ExcerptProp)`
  - `Inference(InferenceProp)`

### struct Value

- Must have generics:
  - `T`
- Must have fields:
  - `inner: T`
  - `dependencies: Vec<Arc<Prop>>` (optional)

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
