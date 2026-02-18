# Simple Curl in Rust

Today you'll learn the fundamentals of Rust by building a simple version of [cURL](https://curl.se/).

## Why Rust?

There are so many great things going for Rust, but here are a few of the main reasons:

- It is fast & compiles into a static binary.
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
beginner-to-intermediate level grasp of the language.

## Setup

[Install Rust here](https://rust-lang.org/tools/install/) with the handy curl
command. There are some extra steps for Windows.

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

This is a good time to install the [rust-analyzer](https://rust-analyzer.github.io/) plugin.

- VSCode: search in marketplace
- Zed: preinstalled
- Neovim, other, etc: may require more setup. Not necessary for this project.

## Step 1: Arguments

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

## Step 2: Using the arguments

We use `.next()` on our args iterator to get the next argument. This means we have to declare the args variable as mutable!

```rust
fn main() {
    let mut args = std::env::args();

    let program_path = args.next();
    let request_type = args.next();
    let url = args.next();
    println!("{program_path:?}, {request_type:?}, {url:?}");
}
```

Note that we get an Option<String> from the iterator, instead of `String` or `null`. `null` doesn't exist in rust!

## Step 3: using dependencies!

### Dependencies

Add this line to your `cargo.toml` under `[dependencies]`:

```
reqwest = {version = "0.12", features = ["blocking"] }
```

This tells `cargo` that your project depends on things in `reqwest` library.
Libraries are `crates` in Rust. The main crate registry is [crates.io](https://crates.io/crates/reqwest).

Now let's use this library in our code to actually fire off an HTTP request.

```rust
fn main() {
    let mut args = std::env::args();

    let program_path = args.next().unwrap();
    let request_type = args.next().unwrap();
    let url = args.next().unwrap();
    println!("{program_path:?}, {request_type:?}, {url:?}");

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send().unwrap();
    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}
```

## Step 4: Using Structs

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

Armed with this knowledge, let's go ahead and make a struct that represents our input, and make a function that converts `args` to our struct!

```rust
fn main() {
    let args = std::env::args();
    let input = Input::from_args(args);

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(input.url).send().unwrap();
    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

struct Input {
    request_type: String,
    url: String,
}

impl Input {
    fn from_args(mut args: std::env::Args) -> Input {
        let _program_path = args.next();
        let request_type = args.next().unwrap();
        let url = args.next().unwrap();
        Input { request_type, url }
    }
}
```


## Step 5: Get and Post with enums

We're using a String to represent our request type - this is something that's best done with an enum.

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
In Rust, we can put stuff inside the variants!
```rust
enum Vehicle {
    Train(u8), // train has some number of cars
    Bus, // bus is always one car
}
```

In our project we'll use a basic enum like this:
```rust
fn main() {
    let args = std::env::args();
    let input = Input::from_args(args);

    let client = reqwest::blocking::Client::new();

    let mut response = match input.request_type {
        RequestType::Get => client.get(input.url).send().unwrap(),
        RequestType::Post => client.post(input.url).send().unwrap(),
    };

    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

enum RequestType {
    Get,
    Post,
}

impl RequestType {
    fn from_string(arg: String) -> RequestType {
        if arg == "post" {
            return RequestType::Post;
        }
        return RequestType::Get;
    }
}

struct Input {
    request_type: RequestType,
    url: String,
}

impl Input {
    fn from_args(mut args: std::env::Args) -> Input {
        let _program_path = args.next();
        let request_type = RequestType::from_string(args.next().unwrap());
        let url = args.next().unwrap();
        Input { request_type, url }
    }
}
```

## Step 6: Implementing traits

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

```rust
fn main() {
    let input: Input = std::env::args().into();

    let client = reqwest::blocking::Client::new();

    let mut response = match input.request_type {
        RequestType::Get => client.get(input.url).send().unwrap(),
        RequestType::Post => client.post(input.url).send().unwrap(),
    };

    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

enum RequestType {
    Get,
    Post,
}

impl From<String> for RequestType {
    fn from(arg: String) -> RequestType {
        if arg == "post" {
            return RequestType::Post;
        }
        return RequestType::Get;
    }
}

struct Input {
    request_type: RequestType,
    url: String,
}

impl From<std::env::Args> for Input {
    fn from(mut args: std::env::Args) -> Self {
        let _program_path = args.next();
        let request_type = args.next().unwrap().into();
        let url = args.next().unwrap();
        Input { request_type, url }
    }
}
```

## Step 7: Request body
We want our Post requests to have some body. Let's add this as a final optional argument to our input struct:
```rust
struct Input {
    request_type: RequestType,
    url: String,
    body: Option<String>,
}

impl From<std::env::Args> for Input {
    fn from(mut args: std::env::Args) -> Self {
        let _program_path = args.next();
        let request_type = args.next().unwrap().into();
        let url = args.next().unwrap();
        let body = args.next();
        Input {
            request_type,
            url,
            body,
        }
    }
}
```
Note that we're not unwrapping this option because it might not always be there!

Now we need to send this body inside our post request if it's there. This is a sticky state management problem! Thankfully `match` comes in handy.

Remember how we can use `match` on an enum and it makes sure we always handle each possibility? Well, we can use match on combinations of enums and guarantee that all combinations are accounted for:

```rust
fn main() {
    let input: Input = std::env::args().into();

    let client = reqwest::blocking::Client::new();

    let request = match (input.request_type, input.body) {
        (RequestType::Get, _) => client.get(input.url),
        (RequestType::Post, Some(body)) => client.post(input.url).body(body),
        (RequestType::Post, None) => client.post(input.url),
    };

    ...
}
```

`match (input.request_type, input.body)` says that we're matching a pair, or `tuple` of arguments. Each branch in the match statement contains a pair of corresponding outcomes. And it's completely exhaustive - if we don't account for everything the compiler will yell at us. This is a great way to make sure you've thought about all the possible states of your program.

## Step 8: Install Your Rust Binary

Just run `cargo install --path .`

You can now invoke your app by running `curlrs`! It works anywhere in your computer, not just in the project directory.

To uninstall your app, just run `cargo uninstall curlrs`.


## Next Steps
Try out the [Rustlings](https://rustlings.rust-lang.org/) exercises!
