# dbg-pls

A `Debug`-like trait for rust that outputs properly formatted code

## Usage in libraries

Add to your `Cargo.toml`
```toml
dbg-pls = "*"
```

Add to your types

```rust
#[derive(dbg_pls::DebugPls)]
```

## Usage for applications

Add to your `Cargo.toml`
```toml
dbg-pls = { version = "0.1", features = ["pretty"] }
```

And print using `debug`, eg

```rust
println!("{}", debug(&value));
```

## Features

* `derive` - enables the `#[derive(DebugPls)]` derive (default)
* `pretty` - enables the `debug` function for pretty printing
* `colors` - enables the `color` function for syntax highlighted printing

## Example

```rust
use dbg_pls::{debug, DebugPls};

#[derive(DebugPls, Copy, Clone)]
pub struct Demo {
    foo: i32,
    bar: &'static str,
}

let mut val = [Demo { foo: 5, bar: "hello" }; 10];
val[6].bar = "Hello, world! I am a very long string";

println!("{}", debug(&val));
```
Outputs
```text
[
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo {
        foo: 5,
        bar: "Hello, world! I am a very long string",
    },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
]
```

## Example (highlighting)

```rust
use dbg_pls::{color, DebugPls};

#[derive(DebugPls, Copy, Clone)]
pub struct Demo {
    foo: i32,
    bar: &'static str,
}

let mut val = [Demo { foo: 5, bar: "hello" }; 10];
val[6].bar = "Hello, world! I am a very long string";

println!("{}", color(&val));
```
Outputs:

![](readme/highlighted.png)
