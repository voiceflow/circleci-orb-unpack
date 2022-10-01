use std::{path::PathBuf, str::FromStr};

use orb_unpack::unpack_from_file;

// runs the unpacking function from tests/inputs/{name} and compares the output to tests/expected/{name}
fn run_and_check_output(name: &str) {
    let source = PathBuf::from_str(&format!("./tests/inputs/{name}.yaml")).unwrap();
    let dest = PathBuf::from_str(&format!("./tests/outputs/{name}")).unwrap();
    unpack_from_file(source, dest.clone()).unwrap();
    assert!(!dir_diff::is_different(format!("./tests/expected/{name}"), dest).unwrap());
}

#[test]
#[should_panic(expected = "expected 1 document in file, but found 0")]
fn empty_input() {
    run_and_check_output("empty");
}

#[test]
fn all_components() {
    run_and_check_output("all_components");
}

#[test]
fn only_jobs() {
    run_and_check_output("only_jobs");
}

#[test]
fn one_executor() {
    run_and_check_output("one_executor");
}

#[test]
fn only_commands() {
    run_and_check_output("only_commands");
}

#[test]
fn no_components() {
    run_and_check_output("no_components");
}
