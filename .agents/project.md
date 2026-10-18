# simplog repo spec

- Every Rust source file:
  - May have the following declarations:
    - `use nu_path::RelativePathBuf;`
    - `use interval_zoo::IntervalStrictFiniteIncInc;`

## package simplog

- Every struct:
  - Must have a corresponding `macro_rules!` that passes [Macro constructor for T requirements](#macro-constructor-for-t-requirements)
  - Must have a corresponding `macro_rules!` that passes [Macro constructor for Arc<T> requirements](#macro-constructor-for-arct-requirements)
- Every [envelope](#envelope-enum):
  - For each variant:
    - Must have a corresponding `macro_rules!` that passes [Macro constructor for enum E variant V requirements](#macro-constructor-for-enum-e-variant-v-requirements)

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
  - `path: RelativePathBuf`
  - `line_range: IntervalStrictFiniteIncInc<u64>`

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
  - `dependencies: Vec<Arc<Prop>>`

### enum Prop

- Must have variants:
  - `Excerpt(ExcerptProp)`
  - `Inference(InferenceProp)`

### struct Value

- Must have generics:
  - `T`
- Must have fields:
  - `inner: T`
  - `dependencies: Vec<Arc<Prop>>`

### Macro constructor for T requirements

- Must have `#[macro_export]`
- Must have a name equal to `{snake_case(T)}`
- Every arm:
  - Must call a `T` constructor
  - Must call `.clone()` on those constructor args that have a concrete non-generic type that is cheaply clonable (e.g. `Arc`, `Rc`)

### Macro constructor for Arc<T> requirements

- Must have `#[macro_export]`
- Must have a name equal to `arc_{snake_case(T)}`
- Every arm:
  - Must call `Arc::new`
  - Must call a [macro constructor for T](#macro-constructor-for-t-requirements)

### Macro constructor for enum E variant V requirements

- Must have `#[macro_export]`
- Must have a name equal to `{snake_case(E)}_{snake_case(V)}`
- Every arm:
  - Must call `E::V`
  - Must call a [macro constructor for T](#macro-constructor-for-t-requirements) (where `T` is the inner type of variant `V`)

### Macro constructor of Arc of enum E of variant V requirements

- Must have `#[macro_export]`
- Must have a name equal to `arc_{snake_case(E)}_{snake_case(V)}`
- Every arm:
  - Must call `Arc::new`
  - Must call a [Macro constructor for enum E variant V requirements](#macro-constructor-for-enum-e-variant-v-requirements)

### Envelope enum

A enum whose every variant is a tuple with exactly one inner type.
