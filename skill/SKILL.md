---
name: simplog
description: Natural-language reasoning
---

# Simplog

The `simplog_direct` crate provides types for structured reasoning in natural language.

## Definitions

### Proposition

A `simplog_direct::Prop` value.

Aliases: prop.

### Proposition set

A set of [propositions](#proposition).

Aliases: propset.

## Workflows

### Decide on a value of type T

- Construct a value of `simplog_direct::Value` type:
  - Provide your reasoning in `dependencies`

## Write a proposition

- Minimize the amount of code at macro callsite:
  - Use supported shorthand arguments.
  - Omit optional defaults.
  - Rely on `into` calls within macro arms.
- Use lowercase `snake_case` identifiers for statics.
- Add `#![allow(non_upper_case_globals)]` at the module level containing those statics.
- A single-line `statement` value must not end with a period.
- Prefer more specific enum variants.
  - Examples:
    - Prefer `source_github!` to `source_html_page!`.

### Review reasoning

- If you find an incorrect prop:
  - Add your own propset that includes a literal negation of the incorrect prop.
- Check dependency chains for cycles, including across crates.
- Don't check sources unless explicitly asked to.

### Write a reply to the user

- If you found some contradictions:
  - Write "Contradictions:"
  - Write a list of contradictions
    - Each item must be in the following format: "{prop_a_var_name} - {prop_b_var_name}"
