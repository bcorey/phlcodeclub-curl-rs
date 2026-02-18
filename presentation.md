---
title: Introduction to Rust
author: Benjamin Corey
---

# Why Rust?
---
There are so many great things going for Rust, but here are a few of the main reasons:

- It is fast & compiles into a static binary.
- The compiler keeps you from writing certain classes of bugs
- It has a very expressive type system
- The package manager is fast and very familiar

<!-- end_slide -->

# Runs anywhere!
---
- Microcontrollers!
- Web browsers!
- Servers!
- Desktop!
- Mobile!

<!-- end_slide -->

# Options Vs. Nulls
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->
In Rust:
```rust +exec
fn my_func() -> Option<String> {
    None
}

fn main() {
    let foo: Option<String> = my_func();
    foo.len(); // won't compile!
}
```

<!-- column: 1 -->
In Java:
```java +exec
class Main {
    static String myFunc() {
        return null;
    }

    public static void main(String[] args) {
        String foo = myFunc();
        foo.length(); // fails at runtime!
    }
}
```

<!-- reset_layout -->

<!-- end_slide -->

# Handling Options
---

In Rust:
```rust +exec
fn my_func() -> Option<String> {
    None
}

fn main() {
    let foo: Option<String> = my_func();
     match foo {
        Some(s) => println!("{}", s.len()),
        None => println!("nothing!"),
    }
}
```

<!-- end_slide -->

# Results Vs. Exceptions
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->
In Rust:
```rust
fn my_func() -> Result<String, String> {
    Err("Failed".to_string())
}

fn main() {
    let foo = my_func();
    match foo {
        Err(e) => println!("Error: {e}"),
        Ok(s) => println!("{s}"),
    }
}
```

<!-- column: 1 -->
In Java:
```java

class Main {
    static String myFunc() {
        throw Exception;
    }

    public static void main(String[] args) {
        String foo;
        try {
            foo = myFunc();
        } catch(Exception e) {
            return;
        }
        foo.length();
    }
}
```

<!-- reset_layout -->

<!-- end_slide -->

# Types: Enums
---
Enums can have stuff in them!
```rust
enum TransitVehicle {
    Bus {
        capacity: u8,
    },
    Train {
        cars: u8,
        car_capacity: u8
    }
}
```

<!-- end_slide -->

# Types: Structs
---
This is what you might think of as a class in another language.
```rust
// they can have fields
struct Car {
    year: u16,
    model: String,
}

// They can be zero-sized
struct Bollard;

// they can have 'tuple' fields
struct Map(HashMap<Point2D, Item>);

// they can have methods!
impl Map {
    fn place_car(&mut self, point: Point2D, car: Car) {...}
}
```
<!-- end_slide -->

# Types: Numbers
---
We need to know what size everything is! all the time! Even integers and floats.
| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| Architecture-dependent | isize | usize |

And let's not forget `f32` and `f64`.
<!-- end_slide -->


# Types: Strings
---

`String` vs `str`

`str` is a _slice_ of characters. It is a fixed size always - like a Java Array.

`String` is flexibily sized, like a Java ArrayList.
<!-- end_slide -->

# The Borrow Checker
---
```rust +exec
fn main() {
    {
        let x = 5;
    }
    
    println!("{x}");
}
```
<!-- end_slide -->

# The Borrow Checker
---
```rust +exec
fn main() {
    let s = make_string();
    
    println!("{s}");
}

fn make_string() -> str {
    "hello"
}
```
<!-- end_slide -->

# The Borrow Checker
---
```rust +exec
fn main() {
    let s = make_string();
    
    println!("{s}");
}

fn make_string() -> &str {
    "hello"
}
```

<!-- end_slide -->

# The Borrow Checker
---
```rust +exec
fn main() {
    let s = make_string();
    
    println!("{s}");
}

fn make_string() -> &'static str {
    "hello"
}
```
<!-- end_slide -->

# The Borrow Checker 
---
```rust +exec
fn main() {
    let s = make_string();
    
    println!("{s}");
}

fn make_string() -> &'static str {
    "hello"
}
```
<!-- end_slide -->
