## Guidelines

### General

You are a senior Rust software architect.

- Think deeply and make detailed plans before writing code.
- Write high-quality, production-ready, generic, reusable code.

#### Principles

Write code that minimizes losses:

- [Avoid data loss](#avoid-data-loss).
- [Minimize hardcoded data](#minimize-hardcoded-data).
- Minimize memory consumption.
- Minimize execution time.

##### Avoid data loss

- Don't use panicking fns (instead, use checked fns that return a `Result`).
- Don't delete data unless spec explicitly requires it.

##### Minimize hardcoded data

- Don't hardcode values (accept args instead).
- Choose carefully between accepting a param and defining a const:
  - Defs:
    - Params are execution details (user may want to change them).
    - Consts are impl details (user would never want to change them).
  - Examples:
    - Params:
      - Cache TTL.
      - Config path.
    - Consts:
      - Table name.
      - Keyspace name.
  - Recommendations:
    - When in doubt, prefer accepting a param instead of defining a const.

#### Development workflow

- Don't add comments.
- Don't write backwards compatibility code.
- Don't edit files in `.agents`.
- Don't run `git diff` or `git status` solely to review your own work at end of a turn.
- If a later instruction overrides an earlier instruction, follow later instruction.
- If I explicitly ask to update code in a way that deviates from spec, update code, spec.
- If you notice unexpected edits:
  - Then:
    - Keep them.
    - Don't mention them.
- If you see a code item or spec requirement that is unnecessary or incorrect, tell me.
- If you have to apply a workaround:
  - Then:
    - Add a comment next to workaround explaining why it is necessary.
    - Mention workaround in your final report.
- If unexpected behavior impedes progress but is not a blocker (for example: domain is unavailable, program is unavailable, available memory or disk space is too low, cmd runs for an unexpectedly long time or consumes an unexpected amount of resources), mention it in your final msg.
- If task is technically possible but would result in low-quality code:
  - Then:
    - If there is a clearly better alternative:
      - Then: implement it.
      - Else: explain why requested code should not be written.
  - Examples:
    - A task to write `impl From<Foo> for Bar` where `Foo` can't actually be infallibly converted to `Bar` (conversion would require calling `unwrap`, which is bad) — in this case, you should write `impl TryFrom<Foo> for Bar` and reply with "Foo can't be infallibly converted to Bar, so I implemented a fallible conversion instead".
    - A task to write a trait impl that only returns an error — in this case, you should not write trait impl but reply with "trait X can't be implemented for Foo because ..."
- If a sentence starts with "Idea: ":
  - Then:
    - Evaluate it thoroughly.
    - If you agree:
      - Then: implement it.
      - Else: explain why you didn't implement it and brainstorm solutions.
- Use `fd` and `rg` instead of `find` and `grep`.

#### Debugging workflow

- Improve error handling, so that root cause is clearly visible.

#### Recommended crates

- `errgonomic` for error handling.
- `strum` for enum derives.
- `subtype` for newtypes.
- `tempfile` for creating temp dirs or files.

#### Files

- File name must match name of primary item in this file (for example: a file with `struct User` must be named `user.rs`).
- Trait impls must be in same file as target type (for example: put `impl TryFrom<...> for User` in same file as `struct User`, which is `user.rs`).

#### Modules

- Don't use `mod.rs`; use module files with submodules in folder with same name (for example: `user.rs` with submodules in `user` folder).
- When creating a new module, attach it with a `mod` declaration followed by a `pub use` glob declaration. The parent module must re-export all items from child modules. For example:
  ```rust
  fn foo() {}

  mod my_module_name;
  pub use my_module_name::*;
  ```
- Place `mod` and `pub use` declarations at end of file (after code items).
- When importing items that are defined in current crate, import them directly from crate root. For example:
  ```rust
  use crate::foo;
  ```
- Prefer short item paths over long item paths (use a `use` statement), unless it's necessary for disambiguation. For example:
  - Good:
    ```rust
    use clap::ValueEnum;
    use serde::{Deserialize, Serialize};

    #[derive(ValueEnum, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Good (`serde` and `rkyv` prefixes are necessary for disambiguation):
    ```rust
    use clap::ValueEnum;

    #[derive(ValueEnum, From, serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Bad (`clap` and `serde` prefixes are not necessary for disambiguation because their trait names are unique in this module):
    ```rust
    #[derive(clap::ValueEnum, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
- If you need error and result types from `std`, prefer short paths.
  - Examples:
    - `use std::io;`: `io::Result`, `io::Error`.
    - `use std::fmt;`: `fmt::Result`, `fmt::Error`.

#### Visibility

- Items:
  - Prefer `pub` visibility over `pub(crate)` or private visibility.
- Fields:
  - If a struct is a refinement of its fields:
    - Then:
      - Fields must be private.
      - Fns that construct or deserialize struct or mutate its fields must preserve invariant.
    - Else: its fields must be `pub`.

#### Consts

- Define consts only for values used in multiple places (prefer inline values).
- Put consts in `src/constants.rs`.

#### Types

- Always use most specific types (enforce semantic difference through syntactic difference):
  - Use types from existing crates.
    - Use types from `url` crate instead of `String` for URL-related values.
    - Use types from `time` crate instead of `String` or `u64` for datetime-related values.
    - Use types from `phonenumber` crate instead of `String` for phone-related values.
    - Use types from `email_address` crate instead of `String` for email-related values.
    - Use types from `core::num` module that are prefixed with `NonZero` for values that must be non-zero.
  - Search for other existing crates if you need specific types.
  - If you can't find existing crates, define newtypes.
- Every `struct`, `enum`, `union` must be in a separate file (except for error types that implement `Error`).
  - Error types that implement `Error` must be in same files as fns that return them.
- Prefer attaching types as child modules to `src/types.rs`.

#### Fns

- Prefer weakest sufficient trait bound for inputs and associated types (`FnOnce` over `FnMut` over `Fn`, `PartialOrd` over `Ord`, `PartialEq` over `Eq`).
- Implement proper error handling using macros from `errgonomic` crate instead of `unwrap` or `expect` (in normal code and in tests).
  - Use `expect` only in exceptional cases where you can prove that it always succeeds, and provide proof as first arg to `expect` (proof must start with "always succeeds because").
- Prefer streams and iterators:
  - Guidelines for inputs:
    - If fn uses methods that are available only for a specific collection type:
      - Then: prefer taking a specific collection type as input.
      - Else: prefer taking an `impl Stream` or `impl IntoIterator` as input.
  - Guidelines for outputs:
    - If fn return type is naturally an iterator (for example: fn returns output of a `map` or `filter`):
      - Then: prefer returning an `impl Iterator` as output (there's no need to collect into `Vec`).
      - Else: prefer returning a specific collection type as output.
  - Examples:
    - Good:
      ```rust
      /// Good: fn uses only generic `Iterator` methods.
      /// Good: fn naturally returns an iterator.
      pub fn filter_non_empty_strings<'a>(inputs: impl IntoIterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
          inputs.into_iter().filter(|i| i.is_empty().not())
      }

      /// Good: `Vec`-specific `extend_from_slice` requires `Vec` input.
      fn extend_args(mut args: Vec<String>, extra_args: &[String]) -> Vec<String> {
          args.extend_from_slice(extra_args);
          args
      }
      ```
    - Bad:
    - ```rust
      /// Bad: needlessly converts a `Vec` into an iterator, then collects into a `Vec`.
      pub fn filter_non_empty_strings(inputs: Vec<&str>) -> Vec<&str> {
          inputs
              .into_iter()
              .filter(|i| i.is_empty().not())
              .collect::<Vec<_>>()
      }

      /// Bad: forces caller to collect strings into a `Vec`, restricting input types and adding allocations.
      pub fn bar(inputs: Vec<String>) -> Vec<String> {}
      ```
- Prefer implementing and using `From` or `TryFrom` for conversions between types (instead of converting in-place).
- Don't use early-return fast-path guards for empty vectors, iterators, streams (i.e.: don't use `if items.is_empty() { return ...; }`).
- Use destructuring assignment for tuple args, for example: `fn try_from((name, parent_key): (&str, GroupKey)) -> ...`.
- Use iterators instead of for loops. For example:
  - Good:
    ```rust
    use errgonomic::{handle_iter, ErrVec};
    use core::num::ParseIntError;
    use thiserror::Error;

    // Good: iterator pipeline with fallible mapping and error handling.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseNumbersError> {
        use ParseNumbersError::*;
        let iter = inputs.into_iter().map(|s| s.as_ref().trim().parse::<u64>());
        Ok(handle_iter!(iter, InvalidInput))
    }

    #[derive(Error, Debug)]
    pub enum ParseNumbersError {
        #[error("failed to parse {len} numbers", len = source.len())]
        InvalidInput { source: ErrVec<ParseIntError> },
    }
    ```
  - Bad:
    ```rust
    use core::num::ParseIntError;

    // Bad: manual loop and mutable accumulator.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseIntError> {
        let mut out = Vec::new();
        for s in inputs {
            let n = s.as_ref().trim().parse::<u64>()?;
            out.push(n);
        }
        Ok(out)
    }
    ```
- If fn has a clear receiver (`self`, `&self`, `&mut self`):
  - Then: implement it as an associated fn.
  - Else: implement it as a standalone free fn.
- Add a local `use` statement for enums to minimize code size. For example:
  - Good:
    ```rust
    pub fn apply(op: GroupsOp) {
        use GroupsOp::*;
        match op {
            InsertOne(_) => {}
            UpdateOne(_, _) => {}
            DeleteOne(_) => {}
        }
    }
    ```
  - Bad:
    ```rust
    pub fn apply(op: GroupsOp) {
        match op {
            GroupsOp::InsertOne(_) => {}
            GroupsOp::UpdateOne(_, _) => {}
            GroupsOp::DeleteOne(_) => {}
        }
    }
    ```
- Simplify call-site code by accepting `impl Into`. For example:
  - Good:
    ```rust
    pub fn foo(input: impl Into<String>) {
        let input = input.into();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.into()` when passing input arg.
    pub fn foo(input: String) {}
    ```
- Prefer `.map()` instead of `match` when you need to modify value in `Option` or `Result`. For example:
  - Good:
    ```rust
    use core::str::FromStr;
    use core::num::ParseIntError;

    impl FromStr for UserId {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u64>().map(Self::new)
        }
    }
    ```
  - Bad:
  ```rust
  use core::str::FromStr;
  use core::num::ParseIntError;

  impl FromStr for UserId {
      type Err = ParseIntError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
          // Bad: uses more code to express same idea.
          match s.parse::<u64>() {
              Ok(value) => Ok(Self::new(value)),
              Err(error) => Err(error),
          }
      }
  }
  ```
- Use `Self` instead of type name in `impl` items. For example:
  - Good:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          Self::new(duration.as_secs())
      }
  }
  ```
  - Bad:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          UnixTimestamp::new(duration.as_secs())
      }
  }
  ```
- Prefer short method syntax over long trait-path syntax (prefer `value.method(arg)` over `Trait::method(value, arg)`).
- Generic helper fns must be in `src/functions` (one file per fn).
- If Clippy reports `too_many_arguments`:
  - Then:
    - Don't use tuples to silence lint.
    - Consider refactoring code for better separation of concerns.

#### Struct derives

- Derive `new` from `derive_new` crate for types that need `fn new`.
- If struct derives `Getters`:
  - Then:
    - For each field:
      - If its type implements `Copy`, it must have a `#[getter(copy)]` annotation.
  - Examples:
    - Good (`username` has no `#[getter(copy)]` because `String` doesn't implement `Copy`, but `age` has it because `u64` implements `Copy`):
      ```rust
      #[derive(Getters, Into, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
      pub struct User {
        username: String,
        #[getter(copy)]
        age: u64,
      }
      ```

#### Setters

- Use setters that take `&mut self` instead of setters that take `self` and return `Self` (because passing `foo: &mut Foo` is more efficient than passing `foo: Foo` and returning `Foo` through call stack).

#### Enums

- When writing code related to enums, bring variants into scope with a `use Enum::*;` statement at top of file or fn (prefer top of file for data enums and top of fn for error enums).

#### Arithmetic

- Don't use impls of traits `core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Shl, ShlAssign, Shr, ShrAssign}` or their operators unless they don't panic or silently overflow.
- Write and use arithmetic trait impls that don't panic or silently overflow.
- Prefer `checked` versions of arithmetic ops.
- Every call to an `overflowing`, `saturating`, `wrapping` version must have a single-line comment above it that starts with "SAFETY: " and describes why calling this version is safe in this specific case.
- Use items from `num` crate if necessary (for example: to implement a fn that calls arithmetic methods on a generic type).

#### Index access

- Never use operators: `[], []=`.
- Never use traits: `core::ops::{Index, IndexMut}`.
- If you are sure that `get` or `get_mut` will never panic, use `expect` with a proof msg (as described in [Fns](#fns)).

#### Test fn

A fn marked with `#[test]` or `#[tokio::test]`.

- Must return a `Result`.
- Must implement proper error handling via `errgonomic` crate.
- Should use macros from `assertables` crate.
  - Should use `assert_infix` instead of `assert_gt`, `assert_ge`, `assert_lt`, `assert_le`, `assert_eq`.

#### Macros

- In `macro_rules!` expansions:
  - Use full paths without a leading `::` for foreign types (e.g.: `std::sync::Arc`), except those in standard prelude.
  - Use `$crate` paths for types defined in this crate (e.g.: `$crate::MyType`).
- If you see similar code in different places, replace similar code with a shared macro.
- If last arg of a macro arm is variadic (e.g.: `$($item:ident),*`):
  - Then: add a trailing `$(,)?`.
  - Else: don't add a trailing `$(,)?`.

#### Shell

- Prefer short options for cmds used in tool calls.

#### mise.toml

- Each `env` map value:
  - If it should be overridable by the user:
    - Then: it must be a map with `default` key (e.g. `PORT = { default = "3000" }`)
    - Else: it must be a specific value (e.g. `SHUCK_CACHE_DIR = "{{config_root}}/.cache"`)

#### Cargo.toml

- Don't define pkg features with only a single optional dep (such features are already defined by Cargo automatically).
- Use `cargo add` to add deps.
- When adding a dep from crates.io:
  - If pkg is [publishable](#publishable-pkg):
    - Then:
      - Run `cargo add {dependency}@{version}`.
        - Patch component of `{version}` must be 0.
      - Try `cargo update -p {dependency} --precise {version}` to lock that exact version.
        - If dep constraints prevent locking that version:
          - Then:
            - Keep version resolved by Cargo.
            - Add a comment in `Cargo.toml` explaining constraints.
    - Else:
      - Run `cargo add {dependency}` without `{version}`.
- When adding a dep in a workspace:
  - Add it to top-level manifest first (`workspace.dependencies`).
- When adding a dep for a workspace member:
  - Run `cargo add {dependency}` without `{version}` (Cargo will set `workspace = true`).
- When adding a new workspace member: add it to `packages` dir unless specified otherwise.

#### Code style

- Don't enforce a line len limit when writing code, comments, docs.

#### Publishable pkg

A pkg that has a remote whose name contains `public` or `pre-public` and ends with `template`.

### simplog repo spec

- The root manifest must define a virtual workspace without a root package.
- Every package must be located under `packages/`.
- Every Rust source file:
  - May have the following declarations:
    - `use derive_more::{Deref, AsRef};`

#### package simplog_direct

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

##### struct GithubRepo

- Must have fields:
  - `owner: &'static str`
  - `name: &'static str`

##### struct GithubCommit

- Must have fields:
  - `repo: &'static GithubRepo`
  - `oid: &'static str`

##### struct GithubSource

- Must have fields:
  - `commit: &'static GithubCommit`
  - `path: &'static str`
    - /// Caller guarantees a nonempty repository-relative path without `..` components, identifying a file in `commit`.
  - `line_range: (usize, usize)`
    - /// Inclusive, one-based bounds. Caller guarantees `1 <= start <= end <= line_count` for the file in `commit`.
- The `source_github!` macro:
  - Must accept a tuple of endpoints.
  - Must normalize their order.

##### struct HtmlPageSource

- Must have fields:
  - `url: &'static str`

##### enum Source

- Must have variants:
  - `Github(GithubSource)`
  - `HtmlPage(HtmlPageSource)`

##### struct AxiomProp

- Must have fields:
  - `statement: &'static str`
  - `source: Option<Source>` (optional)

##### struct TheoremProp

- Must have fields:
  - `statement: &'static str`
  - `dependencies: &'static [&'static Prop]`

##### enum Prop

- Must have variants:
  - `Axiom(AxiomProp)`
  - `Theorem(TheoremProp)`

##### struct Value

- Must have generics:
  - `T`
- Must have derives:
  - `Deref`
  - `AsRef`
- Must have fields:
  - `inner: T`
  - `dependencies: &'static [&'static Prop]`
- Must not have a constructor macro.

##### Macro declaration of T requirements

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
  - Must use `$crate` paths for package items.
  - Must evaluate each supplied argument once.

##### Macro declaration of enum E of variant V requirements

- Must follow [T declaration requirements](#macro-declaration-of-t-requirements), except as specified below.
- Must be named `{snake_case(E)}_{snake_case(V)}` when `E` is not `Prop`.
- Must be named `{snake_case(V)}` when `E` is `Prop`.
- Must accept the inner type's field values.
- Must support the inner type's optional argument lists.
- Every arm:
  - Must emit `pub static name: E = E::V(...);`.
  - Must construct the inner value with a struct literal.

##### Macro constructor of T requirements

- Must be named `{snake_case(T)}`.
- Must have one arm for every argument list obtained by omitting zero or more trailing optional fields.
- Every arm:
  - Must call a `T` constructor.
  - Must call `.clone()` on concrete, non-generic, non-`Copy` arguments that are cheaply clonable (e.g. `Arc`, `Rc`).

##### Macro constructor of Arc of T requirements

- Must be named `arc_{snake_case(T)}`.
- Must have the same arms as the [T constructor macro](#macro-constructor-of-t-requirements).
- Every arm:
  - Must call the T constructor macro.
  - Must wrap its result in `Arc::new`.

##### Macro constructor of enum E of variant V requirements

- Must be named `{snake_case(E)}_{snake_case(V)}`.
- Must have the same arms as the [T constructor macro](#macro-constructor-of-t-requirements), where `T` is the variant's inner type.
- Every arm:
  - Must call the T constructor macro.
  - Must wrap its result in `E::V`.

##### Macro constructor of Arc of enum E of variant V requirements

- Must be named `arc_{snake_case(E)}_{snake_case(V)}`.
- Must have the same arms as the [variant constructor macro](#macro-constructor-of-enum-e-of-variant-v-requirements).
- Every arm:
  - Must call the variant constructor macro.
  - Must wrap its result in `Arc::new`.

##### Envelope enum

An enum whose variants each contain exactly one tuple field.

#### package simplog_lazy_lock

- Must follow the `simplog_direct` requirements with the following changes.
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
  - Must use `GitOid`, an alias for `gix_hash::ObjectId`.
- The `github_commit!` and `source_github!` macros:
  - Must accept the referenced declaration's path without `&`.
- The `github_commit!` macro:
  - Must accept a hexadecimal string literal encoding a 20-byte SHA-1 object ID.
  - Must validate literal syntax and decoded length at compile time using `hex_literal::hex!` and `GitOid::Sha1`.
  - Must also accept a `GitOid` expression.
- The `source_html_page!` macro:
  - Must accept the string-literal argument accepted by `url_macro::url!`.
  - Must construct the URL with `url_macro::url!`.

## Project files

### mise.toml

```toml
min_version = "2026.7.13"

[settings]
idiomatic_version_file_enable_tools = ["rust"]
task.output = "keep-order"

[tools]
node = "24.15.0"
deno = "1.46.1"
fnox = "1.33.1"
fd = "10.4.2"
"github:ewhauser/shuck" = "0.2.2"
"aqua:nushell/nushell" = "0.116.1"
"cargo:nu-lint" = "1.8.0"
"github:nushell/nufmt" = "0.1.6"
"aqua:rvben/rumdl" = "0.3.0"
"npm:@commitlint/config-conventional" = "19.6.0"
"npm:@commitlint/cli" = "19.6.0"
"npm:@commitlint/types" = "19.5.0"
"npm:skills" = "1.5.24"
"cargo:https://github.com/DenisGorbachev/cargo-insert-docs" = { version = "rev:9bccf15cc367a50d2652b0eaf5da7faf5929c666", crate = "cargo-insert-docs", locked = true }
"cargo:https://github.com/DenisGorbachev/repotasks" = { version = "rev:ae4408f4f1a8f90ee184539e1f599f50e90eb43e", locked = true }
"cargo:cargo-hack" = "0.6.33"
"cargo:cargo-nextest" = "0.9.145"
"cargo:cargo-expand" = "1.0.114"
"cargo:taplo-cli" = "0.10.0"
"cargo:sd" = "1.0.0"
"cargo:cargo-edit" = "0.13.13"

[hooks]
postinstall = { task = "git:install-hooks" }

[env]
SHUCK_CACHE_DIR = "{{config_root}}/.cache"

[tasks."build"]
run = "cargo build --workspace"

[tasks."check"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test"] }]

[tasks."check:ci"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test:ci"] }]

[tasks."test"]
depends = ["test:code", "test:docs"]

[tasks."lint"]
depends = ["lint:name", "lint:configs", "lint:code", "lint:code:style", "lint:shell", "lint:nu", "lint:docs", "lint:generated", "lint:reports"]

[tasks."lint:name"]
run = [{ task = "fix:name", args = ["--check"] }]

[tasks."lint:configs"]
depends = ["lint:configs:cargo", "lint:configs:fnox"]

[tasks."lint:configs:cargo"]
run = [{ task = "fix:cargo", args = ["--check"] }]

[tasks."lint:configs:fnox"]
run = [{ task = "fix:fnox" }]

[tasks."lint:code"]
run = "cargo hack clippy --locked --workspace --all-targets --each-feature -- -D warnings"

[tasks."lint:code:style"]
run = "cargo fmt --all -- --check"

[tasks."lint:shell"]
# Shuck 0.2.2 scans Git-ignored local files, so exclude them explicitly.
run = '''shuck --config "lint.source-paths = ['$HOME']" check --extend-exclude '*.local.*' .'''

[tasks."lint:docs"]
run = "rumdl check"

[tasks."lint:generated"]
run = [{ task = "md:gen", args = ["AGENTS.ts"] }, { task = "md:gen", args = ["README.ts"] }]

[tasks."test:code"]
depends = ["test:code:regular", "test:code:integration"]

[tasks."test:code:regular"]
run = "fnox --profile test exec --replace -- cargo nextest run --locked --workspace --all-features --no-tests warn --cargo-quiet --show-progress none --no-input-handler --status-level fail --final-status-level flaky --no-fail-fast"

[tasks."test:code:integration"]
# `--test-threads 1` because integration tests must be run sequentially
run = [{ task = "test:code:regular", args = ["--ignore-default-filter", "--test-threads", "1", "integration_tests::"] }]

[tasks."test:docs"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "cargo test --locked --workspace --doc --all-features --no-fail-fast --quiet"

[tasks."test:ci"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "fnox --profile test exec --replace -- cargo hack test --locked --workspace --feature-powerset --keep-going --no-fail-fast --quiet -- --skip integration_tests::"

[tasks."pre-commit"]
alias = "pre-merge-commit"
run = [{ task = "git:validate-commit" }]

[tasks."commit-msg"]
run = 'mise run --output interleave commitlint -- --edit "$@"'

[tasks."fix"]
depends = ["cargo:validate-config"]
run = [{ task = "fix:aux" }]

[tasks."fix:aux"]
depends = ["fix:configs", "fix:docs", "fix:agents", "fix:readme"]

[tasks."fix:configs"]
depends = ["fix:cargo", "fix:fnox"]

[tasks."fix:code"]
depends = ["fix:name", "fix:code:style", "fix:shell", "fix:nu"]

[tasks."fix:nu"]
run = [{ task = "nu", args = ["nufmt", "--shebang"] }]

[tasks."fix:shell"]
run = [{ task = "lint:shell", args = ["--fix"] }]

[tasks."fix:code:warnings"]
depends = ["fix:cargo"]
# second pass is needed because "cargo clippy --fix" exits with 0 even if some warnings remain
env = { __CARGO_FIX_YOLO = 'yeah' }
run = [
    "cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged",
    { task = "lint:code" },
]

[tasks."fix:code:style"]
# Run after `fix:code:warnings` because both tasks modify the same code files.
depends = ["fix:code:warnings"]
run = "cargo fmt --all"

[tasks."fix:docs"]
depends = ["fix:agents", "fix:readme"]
# use `rumdl check --fix` instead of `rumdl fmt` because `rumdl check --fix` exits with 1 if errors remain (since v0.1.0)
run = [{ task = "lint:docs", args = ["--fix"] }]

[tasks."fix:agents"]
# "fix:agents" depends on "fix:code" because it reads the code files
depends = ["fix:name", "fix:configs", "fix:code"]
run = [{ task = "gen:agents" }]

[tasks."gen:readme"]
run = [{ task = "md:gen", args = ["README.ts", "--fix"] }]

[tasks."gen:agents"]
run = [{ task = "md:gen", args = ["AGENTS.ts", "--fix"] }]

[tasks."commitlint"]
run = "commitlint --extends \"$(mise where npm:@commitlint/config-conventional)/node_modules/@commitlint/config-conventional/lib/index.js\""

[tasks."agent:docs:list"]
run = "[ -d .agents/docs ] && find .agents/docs -type f -print || true"
output = "interleave"
quiet = true
```

### fnox.toml

```toml
#:schema https://fnox.jdx.dev/schema.json

if_missing = "error"
env = "exec"

[providers]
keychain = { type = "keychain", service = "simplog" }
pass = { type = "password-store", prefix = "simplog/" }
age = { type = "age", recipients = [
    "age1sf4r4amev2svqr6llwg8hgtz9n7p5qdh7hh0mavcshzfrmgfduksnq3hql",
    "age1605gsnxpe536sprwccyumq74veg0g80u55n8ggems0t8deau6qdsfnq3m3"
] }
```

### Cargo.toml

```toml
[workspace]
members = ["packages/simplog_direct", "packages/simplog_lazy_lock"]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.93.1"
repository = "https://github.com/DenisGorbachev/simplog"
keywords = []
categories = []
exclude = [
    ".*",
    "*.local.*",
    "doc/dev",
    "specs",
    "AGENTS.ts",
    "CargoMetadata.ts",
    "README.ts",
    "AGENTS*.md",
    "deno.lock",
    "deno.json",
    "clippy.toml",
    "commitlint.config.mjs",
    "fnox.toml",
    "mise.toml",
    "nu-lint.toml",
    "nufmt.nuon",
    "rumdl.toml",
    "shuck.toml",
    "rustfmt.toml",
    "rust-toolchain.toml",
    "release-plz.toml",
    "skills-lock.json",
    ".yolobox"
]

[workspace.metadata.details]
name = "simplog"
title = ""
readme = { generate = false }

[workspace.dependencies]
derive-new = "0.7.0"
derive_more = { version = "2.1.1", features = ["full"] }
gix-hash = { version = "0.28.0", features = ["sha1"] }
hex-literal = "1.1.0"
url = "2.5.8"
url-macro = "0.2.5"

[workspace.lints.rust]
redundant_imports = "deny"
unused_import_braces = "deny"
# unused_qualifications must not be "deny" because our code style has multiple `use Foo::*;`, and some macros (derive_more::Display, strum::Display, strum::EnumString) produce code with full qualifications
# unused_qualifications = "deny"

[workspace.lints.clippy]
# Rust 1.100 reports redundant fields inside derive macros: https://github.com/rust-lang/rust-clippy/issues/17525.
redundant_field_names = "allow"
absolute_paths = "deny"
arithmetic_side_effects = "deny"
```

### packages/simplog_direct/Cargo.toml

```toml
[package]
name = "simplog_direct"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
exclude.workspace = true

[package.metadata.details]
title = ""

[lints]
workspace = true

[dependencies]
derive-new.workspace = true
derive_more.workspace = true
gix-hash.workspace = true

[[example]]
name = "socrates"
path = "examples/socrates.rs"
crate-type = ["lib"]

[[example]]
name = "two_plus_two"
path = "examples/two_plus_two.rs"
crate-type = ["lib"]
```

### packages/simplog_lazy_lock/Cargo.toml

```toml
[package]
name = "simplog_lazy_lock"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
exclude.workspace = true
description = "Natural-language reasoning with lazily initialized propositions"

[lints]
workspace = true

[dependencies]
derive-new.workspace = true
derive_more.workspace = true
gix-hash.workspace = true
hex-literal.workspace = true
url.workspace = true
url-macro.workspace = true

[[example]]
name = "socrates_lazy_lock"
path = "examples/socrates.rs"
crate-type = ["lib"]

[[example]]
name = "two_plus_two_lazy_lock"
path = "examples/two_plus_two.rs"
crate-type = ["lib"]

[[example]]
name = "ll_vs_dc_lazy_lock"
path = "examples/ll_vs_dc/main.rs"
```

### packages/simplog_direct/src/lib.rs

```rust
//! Types for structured logical inference.

mod types;
pub use types::*;
```

### packages/simplog_lazy_lock/src/lib.rs

```rust
mod macros;

mod types;
#[doc(hidden)]
pub use hex_literal::hex as __hex;
pub use types::*;
#[doc(hidden)]
pub use url_macro::url as __url;
```
