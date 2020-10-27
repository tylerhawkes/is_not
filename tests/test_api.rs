#[cfg(test)]
use is_not::{is as is_test, not as not_test};
#[cfg(not(test))]
use is_not::{is as not_test, not as is_test};

#[cfg(debug_assertions)]
use is_not::{is as is_debug, not as not_debug};
#[cfg(not(debug_assertions))]
use is_not::{is as not_debug, not as is_debug};

#[is_test]
#[is_debug]
fn test() {
    println!("test and debug")
}

#[is_test]
#[not_debug]
fn test() {
    println!("test and !debug")
}

#[not_test]
#[is_debug]
fn test() {
    panic!("!test and debug")
}

#[not_test]
#[not_debug]
fn test() {
    panic!("!test and !debug")
}

#[test]
fn test_it() {
  test();
}
