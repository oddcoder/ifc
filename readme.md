# Information Flow Control

This crate implements type system for L~equivalence IFC with delimited release declassifier. Full documentation for the project exists in `docs` folder.


## Quick Start

In `cargo.toml`, add the `ifc` and `ifc-macros` to dependencies

```toml
[dependencies]
ifc = {git = "https://github.com/oddcoder/ifc"}
ifc-macros = {git = "https://github.com/oddcoder/ifc"}
```


## Basic Example

This example will compile sucessfully:

```rust
fn main() {
    ifc_block!{
        let a = 5;
        if a == 5 {
            let x = 5;
            x
        } else {
            let x = 6;
            x
        };
    }
}
```

However, the following example fails:

```rust
fn main() {
    ifc_block!{
        let a = 5;
        if a == 5 {
            let x = 5;
            x
        } else {
            #[IFC(Low)]
            let x = 6;
            x
        };
    }
}
```

The reported error is :

```
error: Cannot declare new low variable or assign to low variable in high context
  --> src\main.rs:12:13
   |
12 |             let x = 6;
   |             ^^^^^^^^^^
   |
help: High context was created here
  --> src\main.rs:7:12
   |
7  |         if a == 5 {
   |            ^^^^^^
help: Low variable is used here.
  --> src\main.rs:12:17
   |
12 |             let x = 6;
   |                 ^
```