# Information Flow Control

This crate implements type system for L~equivalence IFC with delimited release declassifier. Full documentation for the project exists in `docs` folder.


## Quick Start

In `cargo.toml`, add the `ifc` and `ifc-macros` to dependencies

```toml
[dependencies]
ifc = {git = "https://github.com/oddcoder/ifc"}
ifc-macros = {git = "https://github.com/oddcoder/ifc"}
```

The project should work with stable rust. However, at the time of writing this readme, Rust nightly is required to get the detailed error messages.


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


## Usage

To create context where where ifc rules apply, use the macro `ifc_block` as follows:


```rust
ifc_block!{
    // This is code block where ifc rules apply.
}
```

From here one everything we mention goes inside ifc_block macro. By default, all variables are high unless specified otherwise. Low variables require specific marker using rust attributes as follows:

```rust
let this_is_high_varialble_by_default;
#[IFC(Low)]
let this_is_low_varialble;
#[IFC(High)] // this one is not necessary since it is high by default.
let another_high_variable;
```

The following IFC holds:

- You can assign low variable to low variables or high variables.
- You can assign high variables only to another high variable.
- You may assign high variables to low variables if they are attributed with `#[IFC(Declassify)]`.
- Loops, if statements, match statements with high variable in the condition must only assign to high variables in the body.
- Function calls return value is always low variable (due to limited functions support).
- Arguments to function calls cannot be high variables unless the function is attributed with `#[IFC(Unsafe)]`.


## Features Support

- [x] Let statements (e.g. `let x = 5`).
- [x] Assign expressions (e.g. `x = 5`).
- [x] Assign-op experessions (e.g. `x += 5`).
- [x] Binary Operations (e.g. `x + y`).
- [x] Unary Operations (e.g. `-x`).
- [x] Nested blocks (e.g. `{let x = 5; {let x = 5}}`).
- [x] Function calls (e.g. `foobar(x,y,z)`).
- [x] If statements {e.g. `if condition {...} else {...}`}.
- [x] Literals (e.g. `"foobar"`).
- [x] Macros (e.g. `println!("hello world")`).
- [x] Match statements (e.g. `match x {_ => ()}`).
- [x] Parens (e.g. `(x)`).
- [x] While loops {e.g. `while x {}`}.
- [ ] Arrays {e.g. `[1,2,3,4, x]`}.
- [ ] Async Expressions {e.g. `async {...}`}.
- [ ] Await Expression (e.g. `fut.await`).
- [ ] Break statement.
- [ ] Type casting (e.g. `a as u8`).
- [ ] Closures. (e.g. `|e| e + 1`).
- [ ] Continue statement.
- [ ] Fields (e.g. `foo.bar`).
- [ ] For loops (e.g `for i in 0..5 {...}`)
- [ ] Infinite loops (e.g `loop {}`).
- [ ] Method calls (e.g `foo.bar()`).
- [ ] Ranges (e.g `0..5`).
- [ ] Structs, Enums, and function definitions.
- [ ] Full Modules.
