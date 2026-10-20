use simplog::{Value, axiom, theorem, value};

axiom!(ALL_HUMANS_ARE_MORTAL, "All humans are mortal.");
axiom!(SOCRATES_IS_HUMAN, "Socrates is human.");
theorem!(SOCRATES_IS_MORTAL, "Socrates is mortal.", [ALL_HUMANS_ARE_MORTAL, SOCRATES_IS_HUMAN]);

fn main() {
    let is_mortal: Value<bool> = value!(true, [SOCRATES_IS_MORTAL]);
    println!("Socrates is mortal: {}", *is_mortal);
}
