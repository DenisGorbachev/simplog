use simplog_lazy_lock::{Value, axiom, theorem, value};

axiom!(FIXED_GRAPH, "This application knows its proposition texts, source identifiers, and dependency graph at compile time.");
axiom!(REVIEWED_METADATA, "This application accepts caller-reviewed source metadata without runtime parsing.");
axiom!(DC_CAPABILITIES, "Direct construction (DC) can represent fixed text, source identifiers, line bounds, and dependency edges using string literals, tuples, and static references.");
axiom!(LL_CAPABILITIES, "LazyLock (LL) can initialize owned proposition text and typed source metadata at first access.");
axiom!(CHOICE_RULE, "When LL and DC both meet the application's requirements, choose DC if runtime construction adds no required capability.");

theorem!(
    BOTH_MODELS_FIT,
    "Both LL and DC can represent this application's proposition graph.",
    [
        FIXED_GRAPH,
        REVIEWED_METADATA,
        DC_CAPABILITIES,
        LL_CAPABILITIES
    ],
);
theorem!(CHOOSE_DC, "Choose DC: all required proposition data is fixed, and runtime construction or validation is unnecessary.", [BOTH_MODELS_FIT, FIXED_GRAPH, REVIEWED_METADATA, CHOICE_RULE],);

fn main() {
    let use_lazy_lock: Value<bool> = value!(false, [CHOOSE_DC]);
    println!("Decision: {}", if *use_lazy_lock { "LL" } else { "DC" });
}
