#![allow(non_upper_case_globals)]

use simplog::{Value, axiom, github_commit, github_repo, source_github, source_html_page, theorem};

github_repo!(repo, "DenisGorbachev", "simplog");
github_commit!(commit, &repo, "cf597a65d24354ad878d409277de6eb47e798759");
source_github!(manifest, &commit, "Cargo.toml", (57, 58));
source_html_page!(cargo_docs, "https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field");

axiom!(declared_name, "The manifest declares package.name = simplog", Some(manifest));
axiom!(name_role, "Cargo uses package.name to identify the package", Some(cargo_docs));
theorem!(choose_name, "Use simplog to identify this package", [declared_name, name_role]);

fn main() {
    let name = Value::new("simplog", const { &[&choose_name] });
    println!("Package name: {}", *name);
}
