## Simple Curl in Rust
Today you'll learn the fundamentals of Rust by building a simple version of `curl`.

## Why Rust?
Rust is fast & compiles to any target from an embedded controller to servers to WASM for web frontends. It comes with an impressive type system and compiler that help keep you from writing bugs and an easy package management system that will feel right at home to anyone who's used `npm`.

How does it do all this?

Most languages fall into one of two buckets. "fast and unsafe" like C or C++, or "Safety through garbage collection" like Java, Javascript, Go, C#, Kotlin, Clojure, Scala, etc.

C/C++ make you manually allocate memory. You can write very fast code, but it often features security vulnerabilities or reliability issues.

You can think of garbage collection like another program running that is always checking to see when your program is done with the memory you've allocated. It helpfully cleans up after you and frees that memory. But this is expensive!

Rust is both fast and safe. The compiler has special rules that you must follow that eliminate memory vulnerabilities without needing a garbage collector.

## What we'll learn
There's lots of really cool rust stuff. We could talk about it all night. But we'll focus on these fundamentals:
- input
- structs
- enums
- traits
- dependencies

You might have heard rust is very hard. This is partially true - when doing complex stuff rust will ask you think harder about it than in other languages. But you can create very fast, useful software just with a beginner-to-intermediate level grasp of the language.

## Setup
[Install Rust here](https://rust-lang.org/tools/install/) with the handy curl command. There are some extra steps for Windows.

- run `rustc --version` to confirm things are working.
- create your project by running `cargo new my-curl-clone`.
- `cd my-curl-clone`
- `cargo run`

We should see `Hello world!` print to the console.

Congrats! You've just compiled and run your first rust program. But what is `cargo`?
- `cargo` is rust's package manager.
- `cargo new` - create a new rust package.
- `cargo build` - build your package.
- `cargo run` - build (if necessary) and then run.

## Let's get to editing
Open the project folder in your preferred editor. 
- `src/main.rs` - your code lives here!
- `target/` - your compiled code. 
- `cargo.lock` - generated information about the packages your project depends on.
- `cargo.toml` - All your project settings.

Now, in `main.rs`, change the string in the print statement and run `cargo run` to see your new message.

This is a good time to install the [rust-analyzer](https://rust-analyzer.github.io/) plugin.
- VSCode: search in marketplace
- Zed: preinstalled
- Neovim, other, etc: may require more setup. Not necessary for this project.

## 1. Arguments
Let's see how to read arguments from the command line.
```rust
fn main() {
  let args = std::env::args();
  println!("{:#?}", args);
}
```
`println!()` is a `macro` that takes our stuff and shoves it into a format string and prints to the console.
The `:#?` is a little trick we can use to print data that doesn't have `to_string()` implemented for it.

## ? Dependencies

Add this line to your `cargo.toml` under `[dependencies]`:
```
reqwest = {version = "0.12", features = ["blocking"] }
```

We've told cargo that your project depends on things in this library, which in rust are called crates. The main crate registry is [crates.io](https://crates.io/crates/reqwest).

## ? Structs
If one variable won't do.
```rust
struct MyStruct {
  my_field: i32
}
```

## ? Enums

```rust
enum RequestType {
    Get,
    Post,
}
```

## ? Traits
In other languages these might called Interfaces. These are ways to describe a set of behaviors you want your type to have. Sometimes, we can just `derive` the behaviors without having to write any of it by hand!

```rust
#[derive(Debug)]
enum RequestType {...}
```

The Debug trait generates code that tell the computer how to print a rudimentary debug representation of our type to the console.

```rust
impl From<String> for RequestType {...}
```
The From trait is very powerful. In rust, instances of structs are not precious like a java class. transforming your data into a different struct happens all the time and is encouraged by this trait. In this case, we have a `String` and we want a `RequestType`. So, we implement `From<String> for RequestType`. Now we just have to call `into()` on a string to try and convert it into our type.

## Install your rust binary
Just run `cargo install --path .`

You can now invoke your app by typing the package name in `cargo.toml`!

To uninstall your app, just run `cargo uninstall my-app`.
