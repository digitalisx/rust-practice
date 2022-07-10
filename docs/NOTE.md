# Rust Basic

`rustup`

`rustc`

`Cargo`

# Variables

## Mutable / Immutable

### Compile Error Case
```rust
let mut number: i32 = 1;
number = 12;
```

### Normal Case
```rust
let number: i32 = 1;
number = 12;
```

### Constants
```rust
const MAX_POINTS: u32 = 100_000; 
```

### Shadowing
```rust
let x: i32 = 5;
let x: i32 = x * 5;
let x: i32 = x * 10; 
```



mut