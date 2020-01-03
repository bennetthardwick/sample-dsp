use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
    fn mid() -> Self;
    fn amp() -> Self;
    fn peak() -> Self;
    fn trough() -> Self;
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

macro_rules! impl_from_sample_float_unsigned {
    ($type:ty, $($sample:ident),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < $sample::mid() {
                    -(($sample::mid() - value) as $type) / $sample::amp() as $type
                } else {
                    ((value - $sample::mid()) as $type) / $sample::amp() as $type
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
                value as $type / $sample::peak() as $type
            }
        })*
    }
}

macro_rules! impl_from_sample_unsigned_float {
    ($type:ident, $($sample:ty),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value <= 0.0 {
                    $type::mid() - ((-value).min(1.0) * ($type::amp()) as $sample) as $type
                } else {
                    $type::mid() + (value.min(1.0) * ($type::amp()) as $sample) as $type
                }
            }
        })*
    }
}

macro_rules! impl_from_sample_unsigned_signed_bigger {
    ($type:ident, $operator:tt, $amount:tt, $($sample:ty),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < 0 {
                    ((-value) as $type $operator $amount)
                } else {
                    (value) as $type $operator $amount + $type::mid()
                }
            }
        })*
    }
}

macro_rules! impl_from_sample_unsigned_signed_smaller {
    ($type:ident, $operator:tt, $amount:tt, $($sample:ty),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < 0 {
                    ($type::mid() as $sample - (-value) $operator $amount) as $type
                } else {
                    ((value) $operator $amount + $type::mid() as $sample) as $type
                }
            }
        })*
    }
}

// Signed is bigger
macro_rules! impl_from_sample_signed_unsigned_bigger {
    ($type:ident, $operator:tt, $amount:tt, $sample:ident) => {
        impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < $sample::mid() {
                    -(($sample::mid() - value) as $type $operator $amount)
                } else {
                    (value - $sample::mid()) as $type $operator $amount
                }
            }
        }
    }
}

macro_rules! impl_from_sample_signed_unsigned_smaller {
    ($type:ident, $operator:tt, $amount:tt, $sample:ident) => {
        impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                if value < $sample::mid() {
                    ($sample::mid() $operator $amount - value $operator $amount) as $type
                } else {
                    (value $operator $amount - $sample::mid() $operator $amount) as $type
                }
            }
        }
    }
}

macro_rules! impl_from_sample_signed_float {
    ($type:ident, $($sample:ty),*) => {
        $(impl FromSample<$sample> for $type {
            #[inline]
            fn from_sample(value: $sample) -> Self {
                (if value < 0.0 { value.max(-1.0) } else { value.min(1.0) } * $type::peak() as $sample) as $type
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
                if self < $sample::mid() {
                    ($sample::mid() - self) + $sample::mid()
                } else {
                    $sample::mid() - (self - $sample::mid())
                }
            }
        })*
    }
}

macro_rules! impl_from_sample_unsigned_unsigned {
    ($bigger:ident, $amount:tt, $smaller:ident) => {
        impl FromSample<$bigger> for $smaller {
            #[inline]
            fn from_sample(value: $bigger) -> Self {
                if value < $bigger::mid() {
                    $smaller::mid() - (($bigger::mid() - value) / $amount) as $smaller
                } else {
                    $smaller::mid() + ((value - $bigger::mid()) / $amount) as $smaller
                }
            }
        }

        impl FromSample<$smaller> for $bigger {
            #[inline]
            fn from_sample(value: $smaller) -> Self {
                if value < $smaller::mid() {
                    $bigger::mid() - ($smaller::mid() - value) as $bigger * $amount
                } else {
                    $bigger::mid() + (value - $smaller::mid()) as $bigger * $amount
                }
            }
        }
    };
}

macro_rules! impl_sample_float {
    ($($sample:ty),*) => {
        $(
            impl Sample for $sample {
                fn mid() -> Self {
                    0.0
                }
                fn amp() -> Self {
                    1.0
                }
                fn peak() -> Self {
                    1.0
                }

                fn trough() -> Self {
                    -1.0
                }
            }
        )*
    }
}

macro_rules! impl_sample_unsigned {
    ($($sample:ident),*) => {
        $(
            impl Sample for $sample {
                fn mid() -> Self {
                    Self::amp() + 1
                }
                fn amp() -> Self {
                    core::$sample::MAX / 2
                }
                fn peak() -> Self {
                    core::$sample::MAX
                }

                fn trough() -> Self {
                    1
                }
            }
        )*
    }
}

macro_rules! impl_sample_signed {
    ($($sample:ident),*) => {
        $(
            impl Sample for $sample {
                fn mid() -> Self {
                    0
                }
                fn amp() -> Self {
                    (core::$sample::MAX / 127) * 127
                }
                fn peak() -> Self {
                    (core::$sample::MAX / 127) * 127
                }

                fn trough() -> Self {
                    -(core::$sample::MAX / 127) * 127
                }
            }
        )*
    }
}

// A whole lot of code
impl_into_sample!(u8, i8, u16, i16, u32, i32, f32, f64);

impl_from_sample_float_unsigned!(f32, u8, u16, u32);
impl_from_sample_float_unsigned!(f64, u8, u16, u32);

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

impl_from_sample_unsigned_signed_bigger!(u32, *, 2, i16);
impl_from_sample_unsigned_signed_bigger!(u32, *, 4, i8);
impl_from_sample_unsigned_signed_bigger!(u16, *, 2, i8);

impl_from_sample_unsigned_signed_bigger!(u32, *, 1, i32);
impl_from_sample_unsigned_signed_bigger!(u16, *, 1, i16);
impl_from_sample_unsigned_signed_bigger!(u8, *, 1, i8);

impl_from_sample_unsigned_signed_smaller!(u16, /, 2, i32);
impl_from_sample_unsigned_signed_smaller!(u8, /, 2, i16);
impl_from_sample_unsigned_signed_smaller!(u8, /, 16909320, i32);

impl_flip_sample_neg!(f64, f32, i32, i16, i8);
impl_flip_sample_unsigned!(u32, u16, u8);

impl_from_sample_unsigned_unsigned!(u32, 2, u16);
impl_from_sample_unsigned_unsigned!(u32, 4, u8);
impl_from_sample_unsigned_unsigned!(u16, 2, u8);

impl_from_sample_signed_float!(i8, f64, f32);
impl_from_sample_signed_float!(i16, f64, f32);
impl_from_sample_signed_float!(i32, f64, f32);

impl_from_sample_signed_unsigned_bigger!(i32, *, 16909320, u8);
impl_from_sample_signed_unsigned_bigger!(i32, *, 4, u16);
impl_from_sample_signed_unsigned_bigger!(i32, *, 1, u32);

impl_from_sample_signed_unsigned_bigger!(i16, *, 2, u8);
impl_from_sample_signed_unsigned_bigger!(i16, *, 1, u16);

impl_from_sample_signed_unsigned_smaller!(i8, *, 1, u8);

impl_from_sample_signed_unsigned_smaller!(i8, /, 2, u16);
impl_from_sample_signed_unsigned_smaller!(i8, /, 4, u32);
impl_from_sample_signed_unsigned_smaller!(i16, /, 2, u32);

impl_sample_float!(f32, f64);
impl_sample_unsigned!(u32, u16, u8);
impl_sample_signed!(i32, i16, i8);

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

#[cfg(test)]
mod tests {
    extern crate std;
    use super::{FlipSample, IntoSample, Sample};
    use sample::Sample as ExtSample;

    macro_rules! test_flips {
        ($($sample:ident),*) => {
            $(
                assert_eq!($sample::mid().flip(), $sample::mid());
                assert_eq!($sample::trough().flip(), $sample::peak());
                assert_eq!($sample::peak().flip(), $sample::trough());
            )*
        }
    }

    macro_rules! sample_equal {
        ($a:tt, $type:tt, $b:tt) => {
            if let Err(e) = std::panic::catch_unwind(|| {
                assert_eq!($a::$type().into_sample::<$b>(), $b::$type());
            }) {
                panic!(
                    "{} from {} to {} failed.",
                    stringify!($type),
                    stringify!($a),
                    stringify!($b)
                );
            }
        };
    }

    macro_rules! c {
    ($($value:tt),*) => ({
        c!(@input ($($value)*) @current () @output ($($value)*));
    });
    (@format $head:tt ($($rest:tt)*)) => ({
        $(
            sample_equal!($head, peak, $rest);
            sample_equal!($head, mid, $rest);
            sample_equal!($head, trough, $rest);
        )*
    });

    (@input $value:tt @current () @output ($head:tt $($tail:tt)*)) => ({
        c!(@input $value @current ({
            c!(@format $head $value);
        }) @output ($($tail )*));
    });
    (@input $value:tt @current ($current:block) @output ($head:tt $($tail:tt)*)) => ({
        c!(@input $value @current ({
            $current
            c!(@format $head $value);
        }) @output ($($tail )*));
    });
    (@input $value:tt @current ($current:block) @output ()) => ({
        $current
    });
}

    #[test]
    fn test_equilibrium() {
        assert_eq!(u8::mid(), u8::equilibrium());
        assert_eq!(u16::mid(), u16::equilibrium());
        assert_eq!(u32::mid(), u32::equilibrium());
        assert_eq!(f32::mid(), f32::equilibrium());
        assert_eq!(f64::mid(), f64::equilibrium());
    }

    #[test]

    fn test_conversion_works() {
        c!(f64, f32, u32, u16, u8, i32, i16, i8);

        assert_eq!(0.0.into_sample::<u8>(), 128);
        assert_eq!(0.0.into_sample::<u16>(), 32768);
        assert_eq!(0.0.into_sample::<u8>().into_sample::<u16>(), 32768);

        assert_eq!(0.5.into_sample::<u8>(), 191);
        assert_eq!(0.0.into_sample::<u8>(), 128);
        assert_eq!(0.2.into_sample::<u32>().into_sample::<f32>(), 0.2);
        assert_eq!(0.65.into_sample::<u32>().into_sample::<f32>(), 0.65);
        assert_eq!(-0.65.into_sample::<u32>().into_sample::<f32>(), -0.65);

        assert_eq!(255u8.into_sample::<f32>(), 1.0);

        assert_eq!(127i8.into_sample::<f32>(), 1.0);
        assert_eq!(-127i8.into_sample::<f32>(), -1.0);
        assert_eq!(255u8.into_sample::<i8>(), 127);
        assert_eq!(1u8.into_sample::<i8>(), 127);

        // Some quality is lost when converting 8 bit samples to 32 bit
        assert_eq!(255u8.into_sample::<i32>(), i32::peak());
        assert_eq!(1u8.into_sample::<i32>(), -i32::peak());

        assert_eq!(i32::peak().into_sample::<u8>(), 255);
        assert_eq!((-i32::peak()).into_sample::<u8>(), 1);

        assert_eq!(1.0.into_sample::<u8>(), 255);
        assert_eq!(1.5.into_sample::<u8>(), 255);
        assert_eq!((-1.0).into_sample::<u8>(), 1);
    }

    #[test]
    fn test_flip() {
        test_flips!(f64, f32, u32, u16, u8, i32, i16, i8);
    }
}
