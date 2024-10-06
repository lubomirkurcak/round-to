# round-to
Round floating point to integer.

### Usage
You can round to `i32` and `i64` explicitly:
```rust
use round_to::*;

assert_eq!(0.4.round_to_i32(), 0);
assert_eq!(0.6.round_to_i64(), 1);
```
or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
```rust
use round_to::*;

let a: i8 = 0.4.round_to();
assert_eq!(a, 0);
```
using these modes:
```rust
use round_to::*;

assert_eq!(0.5.round_to_i32(), 0);
assert_eq!(0.5.floor_to_i32(), 0);
assert_eq!(0.5.ceil_to_i32(), 1);
```

### Implementation

Rounding is implemented using [`round_ties_even`](https://doc.rust-lang.org/std/primitive.f32.html#method.round_ties_even). Floor and ceil use [`floor`](https://doc.rust-lang.org/std/primitive.f32.html#method.floor), and [`ceil`](https://doc.rust-lang.org/std/primitive.f32.html#method.ceil). Values are then converted to target integer type using [`as`](https://doc.rust-lang.org/std/keyword.as.html).

### License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
https://www.apache.org/licenses/LICENSE-2.0 or the MIT license
https://opensource.org/licenses/MIT, at your
option.

