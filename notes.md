# Introduction to Rust

## Goals for today

- Get a feel for the Rust language
- Understand advantages, tradeoffs and core concepts
- Write some Rust code
- Rust is a deep language (similar to C++, Go) rather than C
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

## Memory allocation failures

### Example 1

```rust
fn upper(string: &str) -> &str {
    let uppercased = string.to_uppercase();
    return &uppercased
}
```

- Think of this as a `const char*` in C++, which is an immutable read-only variable
- Guaranteed to reference UTF-8
- Has an explicit length
- In this example we get an error because we are referencing a variable, `uppercased`, on the stack, and it doesn't live long enough to be accessed (very common error in Rust)

```rust
let string_slice: &str = "hello";
let string: String = string_slice.to_string();
assert_eq!(string_slice, &string);
```
- `String` is an owned buffer of UTF-8
- `string_slice.to_string()` allocates the variable to the heap
- Overloading is allowed for comparing two variables

### Example 2

```rust
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

```rust
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

```rust
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

## Memory allocation error handling

- Rust does not have concept of exceptions (for error messages))
- Instead, panic is used - a runtime controlled crash where program exits (similar to in Go)
- In Rust, you can register a handler for specifying what should happen when these types of errors occur
- At runtime, compiler will invoke certain panic hooks in response to memory allocation failures (to handle exceptions)
- No exception handling out of the box as Rust tries to be very explicit about heap allocations (whereas exceptions are implicit)

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

## Structures

```rust
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

## Enumerations

```rust
enum Decision {
    Undecided,
    Approved,
    Rejected
}

Decision::Undecided
Decision::Approved
Decision::Rejected


enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click{x: i64, y: i64}
}

WebEvent::PageLoad
WebEvent::KeyPress('j')
WebEvent::Click{x: 150, y: 230}
```

- Enums are very useful and can be used similarly to structs

## Imperative Code

```rust
fn hypotenuse(a: f64, b: f64) -> f64 {
  let a_squared = a * a;
  let b_squared = b.powi(2);
  let sum = a_squared + b_squared;
  sum.sqrt()
}

fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for current in 2..=n {
        result *= current;
    }
    result
}
```

- Variables are immutable by default so need to use `mut` to specify mutable variable
- `2..=n` specifies range expression from 2 to n (inclusive, as specified by `=`)
- The `factorial` function can overflow if `n` gets too big - can specify behaviour if overflow occurs (e.g. trigger panic condition)

## Options

```rust
enum Option<T> {
  Some(T),
  None,
}

fn holiday_gift(year: i32) -> Option<Gift>
```

- `Option` can be used to return `T` if it exists, otherwise `None` 
- Options needs to be complete, should model entire domain and handle all possible cases, otherwise error will be returned at compile time
- `.unwrap()` can be used to return the value inside `Option`, however if it does not exist an error will be returned (linter can be used to warn against partial functions with incomplete domain)
- Can use `.expect()` to test whether always returns a value (and trigger error if not)

```rust
fn use_holiday_gift_2018() {
  match holiday_gift(2018) {
    Some(gift) => regift(gift, "Sis"),
    None => Memegen::rant(),
  }
}
```

- `match` is the preferred way to specify action for each case

## Functional Influence

```rust
fn factorial2(n: u64) -> Option<u64> {
  match n {
    0 => Some(1),
    _ => match factorial2(n-1) {
      Some(prev) => prev.checked_mul(n),
      None => None,
    }
  }
}
```

- `match` can return a function rather than a value

```rust
fn factorial2(n: u64) -> Option<u64> {
  match n {
    0 => Some(1),
    _ => factorial2(n - 1)?.checked_mul(n),
  }
}

```

- `?` is used for optional chaining that returns back to caller if there is an error, but otherwise continues executing the code
- Very common design pattern

## Macros

```rust
fn regift(gift: Gift, who: &str) {
    println!("Look {}! I get you a {}", who, gift);
}
```
- The `!` part of `println!` tells us that it is a macro

## Reference Types

Three ways to reference:
1. `fn f(t: Thing);` takes (consumes a `Thing`), requires no references to `t`, eg. by other parts of the program since the function will have deallocated `t` by the end of its use of the variable
2. `fn f(t: &mut Thing);` borrows a `Thing` mutably, caller will have a reference to `t` but compiler ensures no *other* references to `t`
3. `fn f(t: &Thing);` borrows a Thing immutably, compiler ensures no mutable references to `t` (shared references with no mutation)

- Reference types are really what makes Rust Rust
- Borrowing is the concept of transferring ownership of a value temporarily to an entity and then returning it to an original owner
- When we call a function and pass in a value, the caller is no longer responsible for deallocating the reference as responsibility is passed on to the function
- If we pass an immutable reference instead then we cannot access the reference anywhere else (since there may be some temporary invariance that is not upheld)
- There is a cloning function `t.clone()` (requires `#[derive(Clone)]`)to recursively copy the object to heap to circumvent errors above - but allows explicitly allocating to heap rather than this being done implicitly
- `Box` is standard type for moving data from stack to heap - eg. `let bt = Box::new(Thing(100));`

## Specifying lifetimes

- Lifetime elision: when passing in references to functions, it is fine to return a reference if its lifetime is shorter than or equal to the lifetime of the input passed in
- When passing in mutliple references as input, can use `'a` to specify which input is used to produce the output

# Example code

## Creating own methods

```rust
impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }
    pub fn is_empty(&self) -> bool {
        match self {
            List::Empty => true,
            _ => false,
        }
    }
}
```

- `new()` is convention used for constructor function
- `is_empty()` takes immutable reference `self` and returns values based on `match`

# Codelab

