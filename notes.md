# Introduction to Rust

## Goals for today

- Rust is a deep language (similar to C++, Go) rather than C
- Get a feel for the Rust language
- Understand advantages, tradeoffs and core concepts
- Write some Rust code
- Won't learn about concurrency
- Won't learn about writing unsafe code (code dealing with raw pointers, where you can deal with memory unsafety)

## Presenter

- Martin Geisler, mgeisler@
- Played around with Rust since 2016
- Worked with C++ since joining in 2019, noticed that a lot of the style guides and best practices for working with C++ encouraged design patterns of Rust, whereas these come out of the box with Rust

# What is Rust?

## Features

- **Powerful**: model domain in great detail
- **Safe**: type system prevents many common issues at compile time (this is a big feature of Rust), ensures that program does not have undefined behaviour (particularly dereferenced pointers) at compile time
- **Fast**: borrowed concept of zero-cost abstractions from C++, meaning you code in high-level way but that has no performance costs as compared to writing code in low-level way (uses LLVM compiler, performance similar to C and C++)
- **Convenient**: strong focus on developer experience, built by people who are used to package managers, project dependencies, built-in error messages
- **Portable**: no runtime required, Rust code compiles to x86 executable file, can even compile down to code that doesn't require any operating system. WebAssembly works very nicely, with a bit of glue and boiler plate code can load Rust code from different platforms.

## Example use cases

- On Android, Rust is used for keystore2, a library used for storing cryptographic keys
- Daemons running on Android (e.g. ones powering Bluetooth) have been increasingly rewritten in Rust

## Type safety and variable lifetime

### Example 1

```
fn upper(string: &str) -> &str {
    let uppercased = string.to_uppercase();
    return &uppercased
}
```
- Think of this as a `const char*` in C++, which is an immutable read-only variable
- Guaranteed to reference UTF-8
- Has an explicit length
- In this example we get an error because we are referencing a variable, `uppercased`, on the stack, and it doesn't live long enough to be accessed (very common error in Rust)

```
let string_slice: &str = "hello";
let string: String = string_slice.to_string();
assert_eq!(string_slice, &string);
```
- `String` is an owned buffer of UTF-8
- `string_slice.to_string()` allocates the variable to the heap
- Overloading is allowed for comparing two variables

### Example 2

```
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
```
- In Rust, the value of any code block is the last value in it (in this case `a` and `b`) - i.e. value of function body becomes return value
- `T` of type `PartialOrd` allows values to be compared
- If-Else statements require both if and else return values to be defined otherwise 

```
#[test]
fn test_min_bad() {
    let a = "aaa".to_string();
    let b = "bbb".to_string();
    let m = min(&(a + "ccc"), &b);
    assert_eq!(m, "aaacc");
}
```
- Rust has built-in sytem for unit testing using `#[test]`
- This test doesn't work in Rust because `(a + "ccc")` is created as a temporary value, but that only lives until the end of the line in Rust, so in the next line it can no longer be accessed

```
#[test]
fn test_min_fixed() {
    let a = "aaa".to_string();
    let b = "bbb".to_string();
    let a_plus_ccc = a + "ccc"
    let m = min(&a_plus_ccc, &b);
    assert_eq!(m, "aaacc");
}
```
- Use `let` binding to increase lifetime of a variable

# Introduction to Rust

## Characteristics

- **Data**: supports a couple of styles of user-defined types
- **Basics**: multi-paradigm language, feels familiar to users of C++ with a touch of functional feel
- **Lifetimes**: tracks object lifetimes in order to provide many of its safety guarantees, purely a concept a compile time not at runtime

## Primitives

- Booleans: `bool`
- Characters: `char`
- Signed Integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned Integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Machine sized integers: `isize`, `usize`
- Floating Point: `f32`, `f64`
- Arrays: `[T; 16]`
- Slices: `&[T]` - used to reference contiguous sequence of elements in a collection rather than whole collection
- String slices: `&str`
- Tuples: `()`, `(T,)`, `(T, U)`, … - unit type `()` is used for functions return type of no meaningful value

Structures

```
struct Character {
    name: String,
    age: u32,
}
let c = Character {
    name: "Rusty".to_string(),
    age: 8
}
c.name // access struct members
c.1 // access c.age using index

let t = Thing; // empty Struct (0 size at runtime, as Rust has no pointers)
```

Enumerations



Panic

- Rust does not have concept of exceptions (for error messages))
- Instead, panic is used - a runtime controlled crash where program exits (similar to in Go)


# Example code


# Codelab
