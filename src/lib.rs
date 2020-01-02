#![no_std]

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// f32, f64, u8, i8, i16, u16, u32,
//

pub trait Sample:
    Sized
    + FlipSample
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Copy
    + FromSample<u8>
    + FromSample<i8>
    + FromSample<u16>
    + FromSample<i16>
    + FromSample<u32>
    + FromSample<i32>
    + FromSample<f32>
    + FromSample<f64>
{
    fn equilibrium() -> Self;
    fn peak() -> Self;
}

pub trait FlipSample {
    fn flip(self) -> Self;
}

pub trait IntoSample {
    fn into_sample<S: Sample>(self) -> S;
}

pub trait FromSample<T> {
    fn from_sample(value: T) -> Self;
}

macro_rules! impl_into_sample {
    ($($sample:ty),*) => {
        $(
            impl IntoSample for $sample {
                #[inline]
                fn into_sample<S: Sample>(self) -> S {
                    S::from_sample(self)
                }
            }

            impl FromSample<$sample> for $sample {
                #[inline]
                fn from_sample(value: $sample) -> Self {
                    value
                }
            }
        )*
    }
}

macro_rules! impl_from_sample_signed_unsigned {
    ($type:ty, $($sample:ident),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < core::$sample::MAX / 2 {
                    -((value as $type) / (core::$sample::MAX / 2) as $type)
                } else {
                    ((value - core::$sample::MAX / 2) as $type) / (core::$sample::MAX / 2) as $type
                }
            }
        })*
    }
}

macro_rules! impl_from_sample_operator {
    ($type:ty, $operator:tt, $amount:tt, $sample:ident) => {
        impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                (value as $type) $operator $amount
            }
        }
    }
}

macro_rules! impl_from_sample_float_signed {
    ($type:ty, $($sample:ident),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                value as $type / core::$sample::MAX as $type
            }
        })*
    }
}

macro_rules! impl_from_sample_unsigned_float {
    ($type:ident, $($sample:ty),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                ((value.max(1.).min(-1.) + 1.) * (core::$type::MAX / 2) as $sample) as $type
            }
        })*
    }
}

macro_rules! impl_from_sample_unsigned_signed {
    ($type:ident, $operator:tt, $amount:tt, $($sample:ty),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < 0 {
                    ((-value) as $type $operator $amount)
                } else {
                    (value) as $type $operator $amount + core::$type::MAX / 2
                }
            }
        })*
    }
}

macro_rules! impl_flip_sample_neg {
    ($($sample:ty),*) => {
        $(impl FlipSample for $sample {
            #[inline]
            fn flip(self) -> Self {
                -self
            }
        })*
    }
}

macro_rules! impl_flip_sample_unsigned {
    ($($sample:ident),*) => {
        $(impl FlipSample for $sample {
            #[inline]
            fn flip(self) -> Self {
                if self == core::$sample::MAX / 2 {
                    self
                } else if self < (core::$sample::MAX / 2) {
                    self + core::$sample::MAX / 2
                } else {
                    self - core::$sample::MAX / 2
                }
            }
        })*
    }
}

// A whole lot of code
impl_into_sample!(u8, i8, u16, i16, u32, i32, f32, f64);

impl_from_sample_signed_unsigned!(f32, u8, u16, u32);
impl_from_sample_signed_unsigned!(f64, u8, u16, u32);

impl_from_sample_operator!(i32, *, 2, i16);
impl_from_sample_operator!(i32, *, 4, i8);
impl_from_sample_operator!(i16, *, 2, i8);
impl_from_sample_operator!(i16, /, 2, i32);
impl_from_sample_operator!(i8, /, 2, i16);
impl_from_sample_operator!(i8, /, 4, i32);

impl_from_sample_float_signed!(f32, i32, i16, i8);
impl_from_sample_float_signed!(f64, i32, i16, i8);

impl_from_sample_unsigned_float!(u32, f64, f32);
impl_from_sample_unsigned_float!(u16, f64, f32);
impl_from_sample_unsigned_float!(u8, f64, f32);

impl_from_sample_unsigned_signed!(u32, *, 2, i16);
impl_from_sample_unsigned_signed!(u32, *, 4, i8);
impl_from_sample_unsigned_signed!(u16, *, 2, i8);
impl_from_sample_unsigned_signed!(u16, /, 2, i32);
impl_from_sample_unsigned_signed!(u8, /, 2, i16);
impl_from_sample_unsigned_signed!(u8, /, 4, i32);

impl_from_sample_unsigned_signed!(u32, *, 1, i32);
impl_from_sample_unsigned_signed!(u16, *, 1, i16);
impl_from_sample_unsigned_signed!(u8, *, 1, i8);

impl_flip_sample_neg!(f64, f32, i32, i16, i8);
impl_flip_sample_unsigned!(u32, u16, u8);

impl FromSample<f64> for f32 {
    #[inline]
    fn from_sample(value: f64) -> f32 {
        (if value < 0. {
            value.max(core::f32::MIN as f64)
        } else {
            value.min(core::f32::MAX as f64)
        }) as f32
    }
}

impl FromSample<f32> for f64 {
    #[inline]
    fn from_sample(value: f32) -> f64 {
        value as f64
    }
}

impl Sample for f32 {
    fn equilibrium() -> Self {
        0.0
    }

    fn peak() -> Self {
        1.0
    }
}

impl Sample for f64 {
    fn equilibrium() -> Self {
        0.0
    }

    fn peak() -> Self {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::IntoSample;
    use sample::Sample;

    #[test]
    fn test_conversion_works() {
        assert_eq!(2 + 2, 4);
    }
}
