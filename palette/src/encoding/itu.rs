use crate::float::Float;

use crate::rgb::{Primaries, RgbSpace, RgbStandard};
use crate::luma::LumaStandard;
use crate::encoding::TransferFn;
use crate::white_point::{D65, WhitePoint};
use crate::yuv::{DifferenceFn, YuvStandard};
use crate::{FloatComponent, FromF64, Yxy};

fn cast<T: FromF64>(float: f64) -> T {
    FromF64::from_f64(float)
}

///The color space of ITU-R BT601 for 525-line.
///
/// See [ITU-R Rec.601].
///
/// [ITU-R Rec.601]: https://www.itu.int/rec/R-REC-BT.601/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT601_525;

/// The color space of ITU-R BT601 for 625-line.
///
/// See [ITU-R Rec.601].
///
/// [ITU-R Rec.601]: https://www.itu.int/rec/R-REC-BT.601/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT601_625;

/// The color space of ITU-R BT709.
///
/// See [ITU-R Rec.709].
///
/// [ITU-R Rec.709]: https://www.itu.int/rec/R-REC-BT.709/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT709;

/// The color space of ITU-R BT2020.
///
/// See [ITU-R Rec.2020].
///
/// [ITU-R Rec.2020]: https://www.itu.int/rec/R-REC-BT.2020/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT2020;

/// This transfer function is shared between `BT601` and `BT709`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Transfer601And709;

/// The transfer function of `BT2020`.
///
/// This is technically very similar to the one of BT.601 and BT.709 but the constants are defined
/// with increased accuracy due to supporting up to 12-bit quantized numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Transfer2020;

/// The Yuv encoding difference functions for BT601.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DifferenceFn601;

/// The Yuv encoding difference functions for BT709.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DifferenceFn709;

/// The Yuv encoding difference functions for BT2020.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DifferenceFn2020;

// See 2.5.1 (page 2). RGB primary luminances.
const BT601_LUMINANCE: (f64, f64, f64) = (0.2990, 0.5870, 0.1140);
// Divisor to renormalize the blue difference signal.
const BT601_BLUE_NORM: f64 = 1.772;
// Divisor to renormalize the red difference signal.
const BT601_RED_NORM: f64 = 1.402;

// Exact primary luminances derived from the color space primaries.
const BT709_LUMINANCE: (f64, f64, f64) = (0.212656, 0.715158, 0.072186);
// Luminances for the sake of exact specification compliance for YUV luminance.
// See 3.2 (page 4)
const BT709_WEIGHTS: (f64, f64, f64) = (0.2126, 0.7152, 0.07212);
// Divisor to renormalize the blue difference signal.
const BT709_BLUE_NORM: f64 = 1.8556;
// Divisor to renormalize the red difference signal.
const BT709_RED_NORM: f64 = 1.5748;

// See Table 4, Derivation of Luminance.
const BT2020_WEIGHTS: (f64, f64, f64) = (0.2627, 0.6780, 0.0593);
// Divisor to renormalize the blue difference.
const BT2020_BLUE_NORM: f64 = 1.8814;
// Divisor to renormalize the red difference.
const BT2020_RED_NORM: f64 = 1.4746;

impl Primaries for BT601_525 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.6300), cast(0.3400), cast(BT601_LUMINANCE.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.3100), cast(0.5950), cast(BT601_LUMINANCE.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.1550), cast(0.0700), cast(BT601_LUMINANCE.2))
    }
}

impl Primaries for BT601_625 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.6400), cast(0.3300), cast(BT601_LUMINANCE.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.2900), cast(0.6000), cast(BT601_LUMINANCE.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.1500), cast(0.0600), cast(BT601_LUMINANCE.2))
    }
}

impl Primaries for BT709 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.6400), cast(0.3300), cast(BT709_LUMINANCE.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.3000), cast(0.6000), cast(BT709_LUMINANCE.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.1500), cast(0.0600), cast(BT709_LUMINANCE.2))
    }
}

impl Primaries for BT2020 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.708), cast(0.292), cast(BT2020_WEIGHTS.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.170), cast(0.797), cast(BT2020_WEIGHTS.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.131), cast(0.046), cast(BT2020_WEIGHTS.2))
    }
}

impl RgbSpace for BT601_525 {
    type Primaries = BT601_525;
    type WhitePoint = D65;
}

impl RgbSpace for BT601_625 {
    type Primaries = BT601_625;
    type WhitePoint = D65;
}

impl RgbSpace for BT709 {
    type Primaries = BT709;
    type WhitePoint = D65;
}

impl RgbSpace for BT2020 {
    type Primaries = BT2020;
    type WhitePoint = D65;
}

impl RgbStandard for BT601_525 {
    type Space = BT601_525;
    type TransferFn = Transfer601And709;
}

impl RgbStandard for BT601_625 {
    type Space = BT601_625;
    type TransferFn = Transfer601And709;
}

impl RgbStandard for BT709 {
    type Space = BT709;
    type TransferFn = Transfer601And709;
}

impl RgbStandard for BT2020 {
    type Space = BT2020;
    type TransferFn = Transfer2020;
}

impl LumaStandard for BT601_525 {
    type WhitePoint = D65;
    type TransferFn = Transfer601And709;
}

impl LumaStandard for BT601_625 {
    type WhitePoint = D65;
    type TransferFn = Transfer601And709;
}

impl LumaStandard for BT709 {
    type WhitePoint = D65;
    type TransferFn = Transfer601And709;
}

impl LumaStandard for BT2020 {
    type WhitePoint = D65;
    type TransferFn = Transfer2020;
}

impl YuvStandard for BT601_525 {
    type RgbSpace = Self;
    type TransferFn = Transfer601And709;
    type DifferenceFn = DifferenceFn601;
}

impl YuvStandard for BT601_625 {
    type RgbSpace = Self;
    type TransferFn = Transfer601And709;
    type DifferenceFn = DifferenceFn601;
}

impl YuvStandard for BT709 {
    type RgbSpace = Self;
    type TransferFn = Transfer601And709;
    type DifferenceFn = DifferenceFn709;
}

impl YuvStandard for BT2020 {
    type RgbSpace = Self;
    type TransferFn = Transfer2020;
    type DifferenceFn = DifferenceFn2020;
}

impl TransferFn for Transfer601And709 {
    fn into_linear<T: Float + FromF64>(x: T) -> T {
        if x <= cast(0.0091) {
            x / cast(4.500)
        } else {
            ((x + cast(0.099)) / cast(1.099)).powf(T::one() / cast(0.45))
        }
    }

    fn from_linear<T: Float + FromF64>(x: T) -> T {
        if x <= cast(0.0018) {
            x * cast(4.500)
        } else {
            x.powf(cast(0.45)) * cast(1.099) - cast(0.099)
        }
    }
}

impl TransferFn for Transfer2020 {
    fn into_linear<T: Float + FromF64>(x: T) -> T {
        let alpha: T = cast(1.09929682680944);
        let beta: T = cast(0.018053968510807);

        if x < beta * cast(4.500) {
            x / cast(4.500)
        } else {
            ((x + alpha - T::one()) / alpha).powf(T::one() / cast(0.45))
        }
    }

    fn from_linear<T: Float + FromF64>(x: T) -> T {
        let alpha: T = cast(1.09929682680944);
        let beta: T = cast(0.018053968510807);

        if x < beta {
            x * cast(4.5)
        } else {
            x.powf(cast(0.45)) * alpha - (alpha - T::one())
        }
    }
}

impl DifferenceFn for DifferenceFn601 {
    fn luminance<T: FloatComponent>() -> [T; 3] {
        // Full intensity matches whitepoint, these are exactly the Y component of primares.
        let (r, g, b) = BT601_LUMINANCE;
        [cast(r), cast(g), cast(b)]
    }

    fn norm_blue<T: FloatComponent>(denorm: T) -> T {
        denorm / cast(BT601_BLUE_NORM)
    }

    fn denorm_blue<T: FloatComponent>(norm: T) -> T {
        norm * cast(BT601_BLUE_NORM)
    }

    fn norm_red<T: FloatComponent>(denorm: T) -> T {
        denorm / cast(BT601_RED_NORM)
    }

    fn denorm_red<T: FloatComponent>(norm: T) -> T {
        norm * cast(BT601_RED_NORM)
    }
}

impl DifferenceFn for DifferenceFn709 {
    fn luminance<T: FloatComponent>() -> [T; 3] {
        // Full intensity matches whitepoint, these are exactly the Y component of primares.
        let (r, g, b) = BT709_WEIGHTS;
        [cast(r), cast(g), cast(b)]
    }

    fn norm_blue<T: FloatComponent>(denorm: T) -> T {
        denorm / cast(BT709_BLUE_NORM)
    }

    fn denorm_blue<T: FloatComponent>(norm: T) -> T {
        norm * cast(BT709_BLUE_NORM)
    }

    fn norm_red<T: FloatComponent>(denorm: T) -> T {
        denorm / cast(BT709_RED_NORM)
    }

    fn denorm_red<T: FloatComponent>(norm: T) -> T {
        norm * cast(BT709_RED_NORM)
    }
}

impl DifferenceFn for DifferenceFn2020 {
    fn luminance<T: FloatComponent>() -> [T; 3] {
        // Full intensity matches whitepoint, these are exactly the Y component of primares.
        let (r, g, b) = BT2020_WEIGHTS;
        [cast(r), cast(g), cast(b)]
    }

    fn norm_blue<T: FloatComponent>(denorm: T) -> T {
        denorm / cast(BT2020_BLUE_NORM)
    }

    fn denorm_blue<T: FloatComponent>(norm: T) -> T {
        norm * cast(BT2020_BLUE_NORM)
    }

    fn norm_red<T: FloatComponent>(denorm: T) -> T {
        denorm / cast(BT2020_RED_NORM)
    }

    fn denorm_red<T: FloatComponent>(norm: T) -> T {
        norm * cast(BT2020_RED_NORM)
    }
}
