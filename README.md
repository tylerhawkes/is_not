# is_not

This crate exports two very simple attribute macros. `is` returns the token stream as is and `not` returns an empty token stream.

These are very similar to the `#[cfg(...)]` attributes, but they can be exported from a crate for things that need to be enabled if
a dependency has certain features enabled. It additionally has the advantage that it also type checks. If you mis-spell something
inside of `#[cfg(mis-spelled)]` it will always be disabled, where these attributes must exist in the namespace.

For example, let's say you want to allow different algorithms or implementations inside of a library that can be turned on and off
and add expensive compile times due to dependencies or will be in the hot path of an application's main loop.

```rust
// algo crate
// This crate ensures that at least one of foo and bar is active using a build script or more fancy cfg attributes.

#[cfg(feature = "foo")]
pub use is_not::{is as is_foo, not as not_foo};
#[cfg(not(feature = "foo"))]
pub use is_not::{not as is_foo, is as not_foo};
#[cfg(feature = "bar")]
pub use is_not::{is as is_bar, not as not_bar};
#[cfg(not(feature = "bar"))]
pub use is_not::{not as is_bar, is as not_bar};

#[cfg(feature = "foo")]
mod foo {
    fn foo(a: Foo) -> i32 {
        // algorithm
    }
    struct Foo {
        // fields
    }
}

#[cfg(feature = "foo")]
mod bar {
    fn bar(a: Bar) -> i32 {
        // algorithm
    }
    struct Bar {
        // fields
    }
}
```

```rust
// app crate
// both of these functions will exist, though compile time flags given to the algo crate will change how they are compiled in the app crate.

// You can ensure that at least one is turned on
#[not_foo]
#[not_bar]
compile_error!("Either foo or bar or both should be enabled in algo crate");

#[is_foo]
fn call_foo() -> i32 {
  algo::foo::foo(algo::foo::Foo{})
}

#[not_foo]
fn call_foo() -> i32 {
  call_bar()
}

#[is_bar]
fn call_bar() -> i32 {
  algo::bar::bar(algo::bar::Bar{})
}

#[not_bar]
fn call_bar() -> i32 {
  call_foo()
}

fn call_algo() -> i32 {
  if crate::use_foo() {
    call_foo()
  } else {
    call_bar()
  }
}
```
