# Simple Curl in Rust

Today you'll learn the fundamentals of Rust by building a simple version of [cURL](https://curl.se/).

## Why Rust?

There are so many great things going for Rust, but here are a few of the main reasons:

- Its fast & compiles into a static binary.
- The compiler has many built-in features that keep you from writing certain
classes of bugs
- It has a very expressive type system that allows you to encode semantics
directly into your types and implementations
- The package manager is fast and very familiar for those who are familiar with
other package managers such as `npm` and `pip`

### The Compiler's Guarantee

Most languages fall into one of two buckets.

- `fast and unsafe`
  - Languages like C, C++, and Zig
  - They force you to manually allocate and free your own memory
  - This code can be _very_ fast when properly written, but is also
  error prone when you get beyond trivial tasks
- `slow and safe`
  - Languages like Python, JavaScript, Java, Go, etc.
  - This is a bit of an overstatement, as these languages are often
  fast enough for many tasks
  - They are generally make use of a runtime that include a `garbage collector`.
  This is a small program that periodically cleans up your memory for you at the
  cost of performance

Rust is both `fast and safe`[^1]. The Rust compiler has special rules that you
must follow that eliminate memory vulnerabilities without needing a significant
runtime, such as a garbage collector.

Over the course of this workshop we will get to know some of the core concepts
of the Rust compiler. Namely, `ownership` and `borrowing`. More on those later.

## What We'll Learn

Rust is a pretty complex language with many helpful features. We could talk about
it all night. However, today we will focus on some fundamental concepts:

- `input`
- `structs`
- `enums`
- `traits`
- `dependencies`

### Quelling Fears of Complexity

You might have heard Rust is very difficult or obtuse language to learn. This is
partially true. Writing complex software in Rust comes with additional cognitive
overhead. You need to shift your thinking to fit with the rules of the Rust compiler.
_However_, you can create very fast, useful software just with a
beginner-to-intermediate level grasp of the language. When in doubt, just
implement the Copy trait! 😹

## Setup

[Install Rust here](https://rust-lang.org/tools/install/) with the handy curl
command. There are some extra steps for Windows 😿.

Once you have the Rust tool chain installed:

- Run `rustc --version` to confirm things are working.
- Create your project by running `cargo new curlrs` (Pronounced _curlers_).
- Run `cd curlrs`
- `cargo run`

We should see `Hello world!` print to the console.

Congrats! You've just compiled and run your first rust program.

## What Is Cargo?

`cargo` is Rust's package manager. It can be used to manage projects (called
packages) and dependencies, and run your code and tests!

Some common and useful `cargo` commands:

- `cargo new`
  - Create a new rust package.
- `cargo build`
  - Build your package.
- `cargo run`
  - Build (if necessary) and then run your package.
- `cargo test`
  - Run your package's tests

## Let's Get to Editing

Open the project folder in your preferred editor.

- `src/main.rs` - your code lives here!
- `target/` - your compiled code.
- `cargo.lock` - generated information about the packages your project depends on.
- `cargo.toml` - All your project settings.

Now, in `main.rs`, change the string in the print statement and run `cargo run`
to see your new message.

This is a good time to install the [rust-analyzer](https://rust-analyzer.github.io/)
plugin. Most editors should have this in their marketplace. I would assume if your
editor doesn't have a marketplace you either don't need the help of the Rust LSP
or already know how to install it.

### Arguments

Let's see how to read arguments from the command line.

```rust
fn main() {
  let args = std::env::args();
  println!("{:#?}", args);
}
```

#### Line by Line Breakdown

- `let args = std::env::args();`
  - This line reads in the arguments passed into our binary invocation and yields
  an iterator of strings.
- `println!("{:#?}", args);`
  - `println!()` is a `macro` that takes our stuff and shoves it into a format string
and prints to the console. It is similar to `printf` but without the C style placeholders
  - `:#?` is a little trick we can use to print data that doesn't have
  `to_string()` implemented for it. This is useful for pretty printing more complex
  types such as `structs` and `enums` without having to manually write code to
  do that.

### Dependencies

Add this line to your `cargo.toml` under `[dependencies]`:

```
reqwest = {version = "0.12", features = ["blocking"] }
```

This tells `cargo` that your project depends on things in `reqwest` library.
Libraries are `crates` in Rust. The main crate registry is [crates.io](https://crates.io/crates/reqwest).

### Structs

`Structs` are container types that hold many individual pieces of data. Sometimes
they hold primitive values like `strings` or `numbers`, but they can also be
_other_ `structs`.

This is a `struct` that holds a single field of type `i32` (Which is a 32bit integer).

```rust
struct MyStruct {
  my_field: i32
}
```

This `struct` includes a field that contains _another_ `struct`.

```rust
struct MyOtherStruct {
  child_struct: MyStruct
}
```

### Enums

`Enums` are very powerful in Rust. Not only are they enumerated values like in
languages like `Typescript`, `Java`, or `Python`, but they act as
**discriminated unions**. That is a fancy way to saying they can act as an
_or_ type. The trick to Rust `enums` is that you can attach additional data
to the value.

For instance, a traditional `enum` might look something like this:

```rust
enum RequestType {
    Get,
    Post,
}
```

However, in Rust we can encode additional data into the `enum` that might only
have data for specific values in the set. In this case the `Post` variant
includes optional `String` data that represents the request body.

```rust
enum RequestType {
    Get,
    Post(Option<String>),
}
```

### Traits

In other languages these might called Interfaces. These are ways to describe a
set of behaviors you want your type to have, often called `methods` in other
languages. Sometimes, we can just _derive_ the behaviors without having to write
any of the implementation by hand!

```rust
#[derive(Debug)]
enum RequestType {...}
```

The _Debug_ `trait` generates code that tells the computer how to print a
rudimentary debug representation of our type to the console.

```rust
impl From<String> for RequestType {...}
```

The _From_ `trait` is **very** powerful. In Rust, instances of `structs` are not
precious like a Java class. Transforming your data into a different `struct`
happens all the time and is encouraged by this `trait`. In this case, we have
a `String` but we _want_ a `RequestType`. So, we implement
`From<String>` for `RequestType`. Now we just have to call `into()` on a string
to try and convert it into our `RequestType`.

## Install Your Rust Binary

Just run `cargo install --path .`

You can now invoke your app by running `curlrs`!

To uninstall your app, just run `cargo uninstall curlrs`.
