/// Ceil floating point to integer.
///
/// # Usage
/// You can ceil to `i32` and `i64` explicitly:
/// ```rust
/// use round_to_int::*;
///
/// assert_eq!(0.7.ceil_to_i32(), 1);
/// assert_eq!(1.3.ceil_to_i64(), 2);
/// ```
/// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
/// ```rust
/// use round_to_int::*;
///
/// let a: i8 = 0.4.ceil_to();
/// assert_eq!(a, 1);
/// ```
pub trait CeilTo<T> {
    /// Ceil floating point to integer.
    ///
    /// # Usage
    /// You can ceil to `i32` and `i64` explicitly:
    /// ```rust
    /// use round_to_int::*;
    ///
    /// assert_eq!(0.7.ceil_to_i32(), 1);
    /// assert_eq!(1.3.ceil_to_i64(), 2);
    /// ```
    /// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
    /// ```rust
    /// use round_to_int::*;
    ///
    /// let a: i8 = 0.4.ceil_to();
    /// assert_eq!(a, 1);
    /// ```
    fn ceil_to(self) -> T;
}

macro_rules! ceil_to {
    ($($t:ty),*) => {
        $(impl CeilTo<$t> for f32 {
            fn ceil_to(self) -> $t {
                self.ceil() as $t
            }
        })*
        $(impl CeilTo<$t> for f64 {
            fn ceil_to(self) -> $t {
                self.ceil() as $t
            }
        })*
    };
}

ceil_to!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

macro_rules! ceil_to_explicit {
    ($trait:ident; $function_name:ident; $t:ty) => {
        /// Ceil floating point to integer.
        ///
        /// # Usage
        /// You can ceil to `i32` and `i64` explicitly:
        /// ```rust
        /// use round_to_int::*;
        ///
        /// assert_eq!(0.7.ceil_to_i32(), 1);
        /// assert_eq!(1.3.ceil_to_i64(), 2);
        /// ```
        /// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
        /// ```rust
        /// use round_to_int::*;
        ///
        /// let a: i8 = 0.4.ceil_to();
        /// assert_eq!(a, 1);
        /// ```
        pub trait $trait {
            /// Ceil floating point to integer.
            ///
            /// # Usage
            /// You can ceil to `i32` and `i64` explicitly:
            /// ```rust
            /// use round_to_int::*;
            ///
            /// assert_eq!(0.7.ceil_to_i32(), 1);
            /// assert_eq!(1.3.ceil_to_i64(), 2);
            /// ```
            /// or implicitly to `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, or `usize`:
            /// ```rust
            /// use round_to_int::*;
            ///
            /// let a: i8 = 0.4.ceil_to();
            /// assert_eq!(a, 1);
            /// ```
            fn $function_name(self) -> $t;
        }
        impl $trait for f32 {
            fn $function_name(self) -> $t {
                self.ceil_to()
            }
        }
        impl $trait for f64 {
            fn $function_name(self) -> $t {
                self.ceil_to()
            }
        }
    };
}

ceil_to_explicit!(CeilToI32; ceil_to_i32; i32);
ceil_to_explicit!(CeilToI64; ceil_to_i64; i64);

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
            assert_eq!(f.ceil() as i32, f.ceil_to_i32());
        });
    }

    #[test]
    fn test_zero() {
        assert_eq!(0i32, 0.0f32.ceil_to());
        assert_eq!(0i64, 0.0f32.ceil_to());
    }

    #[test]
    fn test_zero_64() {
        assert_eq!(0i32, 0.0f64.ceil_to());
        assert_eq!(0i64, 0.0f64.ceil_to());
    }

    #[test]
    fn test() {
        let half = f32::from_bits(0x3f000000);
        let almost_half = f32::from_bits(0x3effffff);
        assert_eq!(half, 0.5);
        assert_ne!(almost_half, 0.5);

        assert_eq!(half.ceil_to_i32(), 1);
        assert_eq!(almost_half.ceil_to_i32(), 1);
        assert_eq!((-half).ceil_to_i32(), 0);
        assert_eq!((-almost_half).ceil_to_i32(), 0);
    }

    #[test]
    fn test_64() {
        let half = f64::from_bits(0x3fe0000000000000);
        let almost_half = f64::from_bits(0x3fdfffffffffffff);
        assert_eq!(half, 0.5);
        assert_ne!(almost_half, 0.5);

        assert_eq!(half.ceil_to_i32(), 1);
        assert_eq!(almost_half.ceil_to_i32(), 1);
        assert_eq!((-half).ceil_to_i32(), 0);
        assert_eq!((-almost_half).ceil_to_i32(), 0);
    }

    #[test]
    fn readme() {
        assert_eq!(0.7.ceil_to_i32(), 1);
        assert_eq!(1.3.ceil_to_i64(), 2);
        let a: i8 = 0.4.ceil_to();
        assert_eq!(a, 1);
    }
}
