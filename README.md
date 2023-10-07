# round-to
Round floating point to integer.

### Usage
You can round to `i32` and `i64` explicitly:
```rust
use round_to::*;

assert_eq!(0.4.round_to_i32(), 0);
assert_eq!(0.5.round_to_i64(), 1);
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

assert_eq!(0.5.round_to_i32(), 1);
assert_eq!(0.5.floor_to_i32(), 0);
assert_eq!(0.5.ceil_to_i32(), 1);
```

### Implementation
As of now, everything is implemented using [`round`](https://doc.rust-lang.org/std/primitive.f32.html#method.round), [`floor`](https://doc.rust-lang.org/std/primitive.f32.html#method.floor), and [`ceil`](https://doc.rust-lang.org/std/primitive.f32.html#method.ceil).
When [`round_ties_even`](https://github.com/rust-lang/rust/issues/96710) is resolved, this crate will elect to use it instead.

In the future, optimized implementations may be added.
