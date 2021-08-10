# Design Decisions and development methodology


## Basic Design decision

`ifc` is designed to be conservative and verbose. All declared variables are assumed to be high unless specifically annotated otherwise. One of the possible decisions for implementing the `ifc` library would be an LLVM module, The advantage of having procedural macros is that it can be plugged into any project without having to touch the toolchain. However, this implies that we will not be able to perform high-level type inference using rust's own type inference system.

Having rust's own type system (or at least one that is on par with it) would allow supporting more complex features of the language, such as IFC on functions, struct fields, or methods. Due to the complexity involved, we decided to not implement any of the features that require a type system for rust.

Rust procedural macros do not provide a module for type inference, and we decided not to implement our own type system. As such, the features that require a rust type system are not implemented. Aside from that, `ifc` has some quirks where things work outside `ifc` but requires more typing information inside `ifc`. Consider the following example:

```rust
let x = 5;
let y  = x;
```

Normally rust should be able to identify that x is of default type integer, and y has the same type of x. But if we add IFC annotation to this snippet as follows:

```rust
ifc_block!{
    #[IFC(Low)]
    let x = 5;
    let y  = x;
};
```

The rust compiler fails to identify the type of `y` with the following error:

```
error[E0283]: type annotations needed
   --> $DIR/define_high_as_low.rs:4:5
    |
4   | /     ifc_block!{
5   | |         #[IFC(Low)]
6   | |         let x = 5;
7   | |         let y  = x;
8   | |     };
    | |______^ cannot infer type for struct `Variable<High, _>`
    |
    = note: cannot satisfy `Variable<High, _>: From<Variable<Low, i32>>`
note: required by `from`
    = note: this error originates in the macro `ifc_block` (in Nightly builds, run with -Z macro-backtrace for more info)
```

The reason that the rust compiler cannot figure out the type of `y` is because the transformed output of the previous code is not as simple as the input code. The generated output from the above code looks as follows:


```rust
let x = ifc::LowVar::new(5);
let y = ifc::HighVar::from(x);
```
The first line constructs the meta-type `LowVar<T>` using the value `5` so the rust compiler knows it is of time `LowVar<u32>` ( feel free to substitute `u32` to the default integer type). The real problem exists in the second line `from` method is not part of the `HighVar`, But it is the trait `std::convert::From`and in our case we are using this particular one `impl<T> From<LowVar<T>> for HighVar<T>` and that the compiler is having hard time figuring out what `T` is. If we rewrite the IFC code block as follows:

```rust
ifc_block!{
    #[IFC(Low)]
    let x: u32 = 5;
    let y: u32  = x;
};
```
Once we introduce types for `x` and `y`, `ifc` uses that information to construct the type information for output of the macro, and we end up with something that looks like this:

```rust
let x: ifc::LowVar<u32> = ifc::LowVar::new(5);
let y: ifc::HighVar<u32> = ifc::HighVar::from(x);
```

This kind of restriction on the typing of rust code that goes into the `ifc` procedural macro is what we get as a side effect for not implementing type inference into `ifc`.

## Development Methodology

`ifc-macros` development was test-driven. We unit tested every feature supported as a measure of correctness, the development had a huge assumption that if features *F*<sub>0</sub> ... *F*<sub>n</sub> are handled correctly, then any combination of these features are also correct.