/// Floor floating point to integer.
///
/// # Usage
/// You can floor to `i32` and `i64` explicitly:
/// ```rust
/// use round_to::*;
///
/// assert_eq!(0.7.floor_to_i32(), 0);
/// assert_eq!(1.3.floor_to_i64(), 1);
/// ```
/// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
/// ```rust
/// use round_to::*;
///
/// let a: i8 = 0.4.floor_to();
/// assert_eq!(a, 0);
/// ```
pub trait FloorTo<T> {
    /// Floor floating point to integer.
    ///
    /// # Usage
    /// You can floor to `i32` and `i64` explicitly:
    /// ```rust
    /// use round_to::*;
    ///
    /// assert_eq!(0.7.floor_to_i32(), 0);
    /// assert_eq!(1.3.floor_to_i64(), 1);
    /// ```
    /// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
    /// ```rust
    /// use round_to::*;
    ///
    /// let a: i8 = 0.4.floor_to();
    /// assert_eq!(a, 0);
    /// ```
    fn floor_to(self) -> T;
}

macro_rules! floor_to {
    ($($t:ty),*) => {
        $(impl FloorTo<$t> for f32 {
            fn floor_to(self) -> $t {
                self.floor() as $t
            }
        })*
        $(impl FloorTo<$t> for f64 {
            fn floor_to(self) -> $t {
                self.floor() as $t
            }
        })*
    };
}

floor_to!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

macro_rules! floor_to_explicit {
    ($trait:ident; $function_name:ident; $t:ty) => {
        /// Floor floating point to integer.
        ///
        /// # Usage
        /// You can floor to `i32` and `i64` explicitly:
        /// ```rust
        /// use round_to::*;
        ///
        /// assert_eq!(0.7.floor_to_i32(), 0);
        /// assert_eq!(1.3.floor_to_i64(), 1);
        /// ```
        /// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
        /// ```rust
        /// use round_to::*;
        ///
        /// let a: i8 = 0.4.floor_to();
        /// assert_eq!(a, 0);
        /// ```
        pub trait $trait {
            /// Floor floating point to integer.
            ///
            /// # Usage
            /// You can floor to `i32` and `i64` explicitly:
            /// ```rust
            /// use round_to::*;
            ///
            /// assert_eq!(0.7.floor_to_i32(), 0);
            /// assert_eq!(1.3.floor_to_i64(), 1);
            /// ```
            /// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
            /// ```rust
            /// use round_to::*;
            ///
            /// let a: i8 = 0.4.floor_to();
            /// assert_eq!(a, 0);
            /// ```
            fn $function_name(self) -> $t;
        }
        impl $trait for f32 {
            fn $function_name(self) -> $t {
                self.floor_to()
            }
        }
        impl $trait for f64 {
            fn $function_name(self) -> $t {
                self.floor_to()
            }
        }
    };
}

floor_to_explicit!(FloorToI32; floor_to_i32; i32);
floor_to_explicit!(FloorToI64; floor_to_i64; i64);

#[cfg(test)]
mod tests {
    use super::*;

    fn test_all_f32s<F: FnMut(f32)>(mut f: F) {
        for i in 0..(1usize << 32) {
            f(f32::from_bits(i as u32));
        }
    }

    #[test]
    #[ignore]
    fn test_all() {
        test_all_f32s(|f| {
            assert_eq!(f.floor() as i32, f.floor_to_i32());
        });
    }

    #[test]
    fn test_zero() {
        assert_eq!(0i32, 0.0f32.floor_to());
        assert_eq!(0i64, 0.0f32.floor_to());
    }

    #[test]
    fn test_zero_64() {
        assert_eq!(0i32, 0.0f64.floor_to());
        assert_eq!(0i64, 0.0f64.floor_to());
    }

    #[test]
    fn test() {
        let half = f32::from_bits(0x3f000000);
        let almost_half = f32::from_bits(0x3effffff);
        assert_eq!(half, 0.5);
        assert_ne!(almost_half, 0.5);

        assert_eq!(half.floor_to_i32(), 0);
        assert_eq!(almost_half.floor_to_i32(), 0);
        assert_eq!((-half).floor_to_i32(), -1);
        assert_eq!((-almost_half).floor_to_i32(), -1);
    }

    #[test]
    fn test_64() {
        let half = f64::from_bits(0x3fe0000000000000);
        let almost_half = f64::from_bits(0x3fdfffffffffffff);
        assert_eq!(half, 0.5);
        assert_ne!(almost_half, 0.5);

        assert_eq!(half.floor_to_i32(), 0);
        assert_eq!(almost_half.floor_to_i32(), 0);
        assert_eq!((-half).floor_to_i32(), -1);
        assert_eq!((-almost_half).floor_to_i32(), -1);
    }
}
