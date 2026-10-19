# simplog repo spec

## package simplog

- Every struct except [`Value`](#struct-value) must have a [declaration macro](#macro-declaration-of-t-requirements).
- Every [envelope](#envelope-enum) variant must have a [declaration macro](#macro-declaration-of-enum-e-of-variant-v-requirements).
- All specified macros must use `macro_rules!` and `#[macro_export]`.
- Implement `Copy` and `Clone` whenever all fields are `Copy`.
- Macros must accept proposition paths `[P, Q]` for `dependencies` and pass `&[&P, &Q]` to the constructor.

### struct GithubRepo

- `owner: &'static str`
- `name: &'static str`

### struct GithubCommit

- `repo: &'static GithubRepo`
- `oid: &'static str`

### struct GithubSource

- `commit: &'static GithubCommit`
- `path: &'static str`
  - /// Caller guarantees a nonempty repository-relative path without `..` components, identifying a file in `commit`.
- `line_range: (usize, usize)`
  - /// Inclusive, one-based bounds. Caller guarantees `1 <= start <= end <= line_count` for the file in `commit`.
- Constructor: accept a tuple of endpoints and normalize their order.

### struct HtmlPageSource

- `url: &'static str`

### enum Source

- `Github(GithubSource)`
- `HtmlPage(HtmlPageSource)`

### struct AxiomProp

- `content: &'static str`
- `source: Option<Source>` (optional)

### struct TheoremProp

- `content: &'static str`
- `dependencies: &'static [&'static Prop]`

### enum Prop

- `Axiom(AxiomProp)`
- `Theorem(TheoremProp)`

### struct Value

- Generic: `T`.
- Derives: `Deref`, `AsRef`.
- `inner: T`
- `dependencies: &'static [&'static Prop]`
- Must have a [constructor macro](#macro-constructor-of-t-requirements).

### Macro declaration of T requirements

- Name: `{snake_case(T)}`.
- Arguments: declaration identifier followed by fields in order; also accept generic arguments when needed.
- One arm per field prefix containing all required fields.
- Emit `pub static NAME: T = ...;`, including generic arguments, by calling a `const fn` constructor.
- Use const defaults for omitted fields and `$crate` paths for simplog items; evaluate each supplied argument once.

### Macro declaration of enum E of variant V requirements

Follow [T declaration requirements](#macro-declaration-of-t-requirements), except:

- Name: `{snake_case(E)}_{snake_case(V)}`, or `{snake_case(V)}` for `Prop`.
- Use the inner type's constructor arguments and prefix arms.
- Emit `pub static NAME: E = E::V(...);` using the inner type's const constructor.

### Macro constructor of T requirements

- Name: `{snake_case(T)}`.
- One arm per field prefix containing all required fields.
- Call a `T` constructor, using `.clone()` for concrete, non-generic arguments that are cheaply clonable (e.g. `Arc`, `Rc`).

### Macro constructor of Arc of T requirements

- Name: `arc_{snake_case(T)}`.
- Match the [T constructor macro](#macro-constructor-of-t-requirements) arms and wrap its call in `Arc::new`.

### Macro constructor of enum E of variant V requirements

- Name: `{snake_case(E)}_{snake_case(V)}`.
- Match the [T constructor macro](#macro-constructor-of-t-requirements) arms and wrap its call in `E::V`, where `T` is the variant's inner type.

### Macro constructor of Arc of enum E of variant V requirements

- Name: `arc_{snake_case(E)}_{snake_case(V)}`.
- Match the [variant constructor macro](#macro-constructor-of-enum-e-of-variant-v-requirements) arms and wrap its call in `Arc::new`.

### Envelope enum

An enum whose variants each contain exactly one tuple field.
