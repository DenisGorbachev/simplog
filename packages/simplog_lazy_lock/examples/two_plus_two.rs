use simplog_lazy_lock::{Value, axiom, theorem, value};

axiom!(SUCCESSORS, "The successor of 0 is 1, of 1 is 2, of 2 is 3, and of 3 is 4.");
axiom!(ADD_ZERO, "For every natural number a, a + 0 = a.");
axiom!(ADD_SUCCESSOR, "For all natural numbers a and b, a + successor(b) = successor(a + b).");

theorem!(TWO_PLUS_ZERO, "2 + 0 = 2.", [ADD_ZERO]);
theorem!(TWO_PLUS_ONE, "2 + 1 = successor(2 + 0) = 3.", [SUCCESSORS, ADD_SUCCESSOR, TWO_PLUS_ZERO]);
theorem!(TWO_PLUS_TWO, "2 + 2 = successor(2 + 1) = 4.", [SUCCESSORS, ADD_SUCCESSOR, TWO_PLUS_ONE]);

fn main() {
    let sum: Value<u64> = value!(4, [TWO_PLUS_TWO]);
    println!("2 + 2 = {}", *sum);
}
