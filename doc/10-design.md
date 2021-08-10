# Design


## `LowVar` And `HighVar`

The data type `Variable` exists at the core of the `ifc` library, it is used to represent both high variables and low variables, the data type `Variable` is defined as follows:

```rust
pub struct Variable<S, T> {
    data: T,
    status: PhantomData<S>,
}
```
Basically, `Variable` has two jobs. The first job is to act as a wrapper for data type `T`. `Variable` is a generic type and it does not store information on its own, it rather stores the data stored in type `T`. The other role of `Variable` is to mark `T` as either a high variable or a low variable. This is achieved using phantom types. Phantom types are data types that the compiler can differentiate at compile-time, but acquires no space at run time.

`HighVar` and `LowVar` are defined as follow:

```rust
pub struct High;
pub struct Low;
pub type HighVar<T> = Variable<High, T>;
pub type LowVar<T> = Variable<Low, T>;
```

It is important to emphasise that this setup provides us with an abstraction that comes with zero runtime overhead because all IFC analysis is done at compile-time, even the data wrapper is a thin wrapper that includes no extra runtime logic.

The rest of the `ifc` crate provides conveniences for dealing with `HighVar` and `LowVar`. Basically, `ifc` implements logic using binary operations, unary operations, and some type castings on both `LowVar` and `HighVar`. These conveniences are in the form of implementing popular rust traits that are A) Safe for IFC systems B) very useful for software development.


## Motivating The Need For Procedural Macros.

One might be falsely lead to think that simply having types for high variable and low variable is sufficient to fully enforce IFC. After all, we can only assign low variables to low variables, high variables to high variables, and cast low variables to high ones.
To show that this is not the case, consider the following example:

```rust
if high_var == 4 {
    low_var = 5;
}
```

For simplicity (this is not really the case), let assuming that rust will take care of typecasting for us implicitly, in the if condition we are checking a `HighVar` and based on the value of that `HighVar` we set a `LowVar`. Using just `ifc` types, the code above would compile successfully, even though it has a huge problem. The problem is that the information about the value of `high_var` is leaked into `low_var`. This is why we need procedural macros to process this code block and other cases where rust's own type system is not strict enough to enforce IFC constraints.


At its core, the `ifc-macros` crate is a recursive function with an internal state. The recursive nature of `ifc-macros` arises from the fact that rust programming language is recursively defined. `ifc-macros` process code blocks and generate equivalent code blocks that follow the IFC type system, or return a descriptive error message when appropriate.