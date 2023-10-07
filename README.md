# round_to_int
Round floating point to integer.

### Usage
You can round to `i32` and `i64` explicitely:
```rust
use round_to_int::*;

assert_eq!(0.4.round_to_i32(), 0);
assert_eq!(0.5.round_to_i64(), 1);
```
or implicitely to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
```rust
use round_to_int::*;

let a: i8 = 0.4.round_to();
assert_eq!(a, 0);
```

### Implementation
As of now, everything is implemented as:
```rust
x.round() as i32
```

In the future, optimized implementations may be added.
