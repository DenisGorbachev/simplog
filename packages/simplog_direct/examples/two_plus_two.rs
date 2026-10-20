#![allow(non_upper_case_globals)]

use simplog_direct::{axiom, theorem};

axiom!(successors, "The successor of 0 is 1, of 1 is 2, of 2 is 3, and of 3 is 4");
axiom!(add_zero, "For every natural number a, a + 0 = a");
axiom!(add_successor, "For all natural numbers a and b, a + successor(b) = successor(a + b)");

theorem!(two_plus_zero, "2 + 0 = 2", [add_zero]);
theorem!(two_plus_one, "2 + 1 = successor(2 + 0) = 3", [successors, add_successor, two_plus_zero]);
theorem!(two_plus_two, "2 + 2 = successor(2 + 1) = 4", [successors, add_successor, two_plus_one]);
