#![allow(non_upper_case_globals)]

use simplog_lazy_lock::{axiom, theorem};

axiom!(all_humans_are_mortal, "All humans are mortal");
axiom!(socrates_is_human, "Socrates is human");
theorem!(socrates_is_mortal, "Socrates is mortal", [all_humans_are_mortal, socrates_is_human]);
