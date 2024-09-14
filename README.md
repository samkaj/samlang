# samlang

Toy language, here be dragons :^)

## Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install) with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Usage

### Compile a file

```console
cargo run -- <source-file>
```

### Run test suite

```console
cargo test # unit tests
./scripts/test.sh # integration tests
```

## Features

Samlang takes inspiration from Go's simplicity and flexibility from Rust. I want readable code which means it should fit somewhere between Python and Java :)

### `let`

Variables can have inferred types, but it is also allowed to explicitly define them

```
let y = 42;
let x: int = 10;
```

### `fn`

Functions are first-class citizens. All function signatures must include return type if not void, then it is optional.

```
fn greet(name: string, day: string) -> void {
    print("hi {}, what a nice {}", name, day);
}

let x = greet;
x("legolas", "friday");

// Functions can be arguments
fn foo(bar: str, baz: |str|: int) -> int {
    return baz(bar);
}

// Functions can return functions
fn yeet(factor: int) -> |int|: int {
    return |x: int|: int {
        return x * factor;
    }
}
```

### `struct`

```
struct Position {
    x: int,
    y: int,
}

let a = Position { x: 10, y: 10 };
let x = a.x;
```

### `interface`, `class`

Not sure if I want OOP concepts, they might be added
