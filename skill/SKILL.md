# Simplog

The `simplog` crate provides types for structured reasoning in natural language.

## Definitions

### Proposition

A `simplog::Prop` value.

Aliases: prop.

### Proposition set

A set of [propositions](#proposition).

Aliases: propset.

## Workflows

### Decide on a value of type T

- Construct a value of `simplog::Value` type:
  - Provide your reasoning in `dependencies`

### Review reasoning

- Don't check sources unless explicitly asked to.
- If you find an incorrect prop:
  - Add your own propset that includes a literal negation of the incorrect prop.

### Write a reply to the user

- If you found some contradictions:
  - Write "Contradictions:"
  - Write a list of contradictions
    - Each item must be in the following format: "{prop_a_var_name} - {prop_b_var_name}"
