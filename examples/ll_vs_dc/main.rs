#![allow(non_upper_case_globals)]

use simplog::{Value, axiom, source_html_page, theorem};

source_html_page!(lazy_lock_docs, "https://doc.rust-lang.org/std/sync/struct.LazyLock.html");

axiom!(fixed_graph, "This application knows its proposition texts, source identifiers, and dependency graph at compile time");
axiom!(reviewed_metadata, "This application accepts caller-reviewed source metadata without runtime parsing");
axiom!(dc_capabilities, "Direct construction (DC) can represent fixed text, source identifiers, line bounds, and dependency edges using string literals, tuples, and static references");
axiom!(ll_capabilities, "LazyLock (LL) can initialize owned proposition text and typed source metadata at first access", Some(lazy_lock_docs));
axiom!(choice_rule, "When LL and DC both meet the application's requirements, choose DC if runtime construction adds no required capability");

theorem!(
    both_models_fit,
    "Both LL and DC can represent this application's proposition graph",
    [
        fixed_graph,
        reviewed_metadata,
        dc_capabilities,
        ll_capabilities
    ]
);
theorem!(choose_dc, "Choose DC: all required proposition data is fixed, and runtime construction or validation is unnecessary", [both_models_fit, fixed_graph, reviewed_metadata, choice_rule]);

fn main() {
    let decision = Value::new(Construction::Direct, const { &[&choose_dc] });
    println!("Decision: {:?}", *decision);
}

mod construction;
pub use construction::*;
