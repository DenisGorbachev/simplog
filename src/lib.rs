//! Logical arguments expressed as propositions and their dependencies.
//!
//! An axiom is accepted as a premise without derivation; an optional source records its origin. A theorem claims to follow from its dependencies. Constructing either does not record a reviewer's approval.
//!
//! A reviewer must be able to reconstruct each inference from its dependencies and shared definitions and logical rules. Additional factual premises must be explicit dependencies. Split unclear inferences into intermediate propositions. Dependencies must not rely on circular reasoning.
//!
//! Record review approvals in comments beside the corresponding bindings.

mod types;
pub use types::*;
