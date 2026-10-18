# simplog repo spec

- Every Rust source file:
  - May have the following declarations:
    - `use nu_path::RelativePathBuf;`
    - `use interval_zoo::IntervalStrictFiniteIncInc;`

## package simplog

- Every `struct`, `enum`, `union` must have a corresponding `macro_rules!` that passes [Arc macro constructor requirements](#arc-macro-constructor-for-type-t-requirements)

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

### Arc macro constructor for type T requirements

- Must have `#[macro_export]`
- Must have a name equal to `T` in snake case
- Every arm:
  - Must call `Arc::new`
  - Must call a `T` constructor in `Arc::new`
  - Must call `.clone()` on those constructor args that have a cheaply clonable type (e.g. `Arc`, `Rc`)
