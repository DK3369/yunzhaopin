//! Explicit numeric conversion helpers.
//!
//! Rust's `as` conversions silently truncate, wrap, or saturate.  These helpers
//! keep the conversion policy visible and attach enough context to diagnose a
//! bad request or a corrupt database value without reproducing it locally.

use std::any::type_name;
use std::fmt::{self, Display};
use std::time::Duration;

use crate::{ApiError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericConversionError {
    context: &'static str,
    value: String,
    target: &'static str,
    reason: String,
}

impl NumericConversionError {
    fn new<T>(context: &'static str, value: impl Display, reason: impl Display) -> Self {
        Self {
            context,
            value: value.to_string(),
            target: type_name::<T>(),
            reason: reason.to_string(),
        }
    }
}

pub fn db_conversion_error<T>(
    context: &'static str,
    value: impl Display,
    reason: impl Display,
) -> sqlx::Error {
    sqlx::Error::Decode(Box::new(NumericConversionError::new::<T>(
        context, value, reason,
    )))
}

impl Display for NumericConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "numeric conversion failed for {}: value {} cannot be represented as {} ({})",
            self.context, self.value, self.target, self.reason
        )
    }
}

impl std::error::Error for NumericConversionError {}

pub fn checked<T, U>(value: U, context: &'static str) -> Result<T, NumericConversionError>
where
    T: TryFrom<U>,
    U: Copy + Display,
    T::Error: Display,
{
    T::try_from(value).map_err(|error| NumericConversionError::new::<T>(context, value, error))
}

pub fn checked_param<T, U>(value: U, context: &'static str) -> AppResult<T>
where
    T: TryFrom<U>,
    U: Copy + Display,
    T::Error: Display,
{
    checked(value, context).map_err(|error| ApiError::param_invalid(error.to_string()))
}

pub fn checked_internal<T, U>(value: U, context: &'static str) -> AppResult<T>
where
    T: TryFrom<U>,
    U: Copy + Display,
    T::Error: Display,
{
    checked(value, context).map_err(ApiError::internal)
}

pub fn checked_db<T, U>(value: U, context: &'static str) -> Result<T, sqlx::Error>
where
    T: TryFrom<U>,
    U: Copy + Display,
    T::Error: Display,
{
    checked(value, context).map_err(|error| sqlx::Error::Decode(Box::new(error)))
}

pub fn checked_db_i64<U>(value: U, context: &'static str) -> Result<i64, sqlx::Error>
where
    i64: TryFrom<U>,
    U: Copy + Display,
    <i64 as TryFrom<U>>::Error: Display,
{
    checked_db(value, context)
}

pub fn checked_db_i32<U>(value: U, context: &'static str) -> Result<i32, sqlx::Error>
where
    i32: TryFrom<U>,
    U: Copy + Display,
    <i32 as TryFrom<U>>::Error: Display,
{
    checked_db(value, context)
}

pub fn checked_db_u64<U>(value: U, context: &'static str) -> Result<u64, sqlx::Error>
where
    u64: TryFrom<U>,
    U: Copy + Display,
    <u64 as TryFrom<U>>::Error: Display,
{
    checked_db(value, context)
}

pub fn checked_db_usize<U>(value: U, context: &'static str) -> Result<usize, sqlx::Error>
where
    usize: TryFrom<U>,
    U: Copy + Display,
    <usize as TryFrom<U>>::Error: Display,
{
    checked_db(value, context)
}

pub fn nonnegative_count<T>(value: T) -> u64
where
    T: TryInto<u64>,
{
    value.try_into().unwrap_or_default()
}

pub fn saturating_count_u32(value: i64) -> u32 {
    u32::try_from(value.max(0)).unwrap_or(u32::MAX)
}

pub fn saturating_count_i32(value: u64) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

pub fn saturating_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

pub fn finite_to_f64(value: f64, context: &'static str) -> AppResult<f64> {
    finite_to_f64_checked(value, context).map_err(ApiError::internal)
}

pub fn finite_to_f64_db(value: f64, context: &'static str) -> Result<f64, sqlx::Error> {
    finite_to_f64_checked(value, context).map_err(|error| sqlx::Error::Decode(Box::new(error)))
}

fn finite_to_f64_checked(value: f64, context: &'static str) -> Result<f64, NumericConversionError> {
    if !value.is_finite() {
        return Err(NumericConversionError::new::<f64>(
            context,
            value,
            "value is not finite",
        ));
    }
    Ok(value)
}

/// Lossy `i64` → `f64` for ratios and price multipliers (pool / chart / VIP).
#[allow(clippy::as_conversions)]
pub fn i64_to_f64(value: i64) -> f64 {
    value as f64
}

/// Connection-pool idle counts are `usize` but the pool size itself is `u32`.
pub fn usize_to_f64(value: usize) -> f64 {
    f64::from(u32::try_from(value).unwrap_or(u32::MAX))
}

/// Display-only `u32` → `f32` (rating `avg_x100` / 100).
#[allow(clippy::as_conversions)]
pub fn u32_to_f32(value: u32) -> f32 {
    value as f32
}

#[allow(clippy::as_conversions)]
fn f64_to_i64(value: f64) -> Option<i64> {
    if (i64::MIN as f64..=i64::MAX as f64).contains(&value) {
        Some(value as i64)
    } else {
        None
    }
}

fn f64_to_u32(value: f64) -> Option<u32> {
    if (0.0..=f64::from(u32::MAX)).contains(&value) {
        #[allow(clippy::as_conversions)]
        Some(value as u32)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatRounding {
    Round,
    Truncate,
}

pub fn finite_f64_to_i64(
    value: f64,
    rounding: FloatRounding,
    context: &'static str,
) -> AppResult<i64> {
    finite_f64_to_i64_checked(value, rounding, context).map_err(ApiError::internal)
}

pub fn finite_f64_to_i64_db(
    value: f64,
    rounding: FloatRounding,
    context: &'static str,
) -> Result<i64, sqlx::Error> {
    finite_f64_to_i64_checked(value, rounding, context)
        .map_err(|error| sqlx::Error::Decode(Box::new(error)))
}

fn finite_f64_to_i64_checked(
    value: f64,
    rounding: FloatRounding,
    context: &'static str,
) -> Result<i64, NumericConversionError> {
    if !value.is_finite() {
        return Err(NumericConversionError::new::<i64>(
            context,
            value,
            "value is not finite",
        ));
    }
    let normalized = match rounding {
        FloatRounding::Round => value.round(),
        FloatRounding::Truncate => value.trunc(),
    };
    f64_to_i64(normalized).ok_or_else(|| {
        NumericConversionError::new::<i64>(context, value, "value is outside the i64 range")
    })
}

pub fn integral_f64_to_u32(value: f64, context: &'static str) -> AppResult<u32> {
    integral_f64_to_u32_checked(value, context).map_err(ApiError::internal)
}

pub fn integral_f64_to_u32_db(value: f64, context: &'static str) -> Result<u32, sqlx::Error> {
    integral_f64_to_u32_checked(value, context)
        .map_err(|error| sqlx::Error::Decode(Box::new(error)))
}

fn integral_f64_to_u32_checked(
    value: f64,
    context: &'static str,
) -> Result<u32, NumericConversionError> {
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(NumericConversionError::new::<u32>(
            context,
            value,
            "value must be a finite whole number",
        ));
    }
    f64_to_u32(value).ok_or_else(|| {
        NumericConversionError::new::<u32>(context, value, "value is outside the u32 range")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_rejects_signed_and_width_overflow() {
        assert!(checked::<u8, _>(-1_i32, "usertype").is_err());
        assert_eq!(checked::<u8, _>(255_i32, "usertype").unwrap(), 255);
        assert!(checked::<u8, _>(256_i32, "usertype").is_err());
        assert_eq!(
            checked::<u32, _>(u64::from(u32::MAX), "did").unwrap(),
            u32::MAX
        );
        assert!(checked::<u32, _>(u64::from(u32::MAX) + 1, "did").is_err());
        assert_eq!(
            checked::<u64, _>(u128::from(u64::MAX), "id").unwrap(),
            u64::MAX
        );
        assert!(checked::<u64, _>(u128::from(u64::MAX) + 1, "id").is_err());
        assert!(checked::<usize, _>(u128::from(u64::MAX) + 1, "buffer.len").is_err());
    }

    #[test]
    fn request_and_database_errors_preserve_context() {
        for (context, value) in [("usertype", -1_i64), ("did", -1), ("id", -1)] {
            let error = checked_param::<u32, _>(value, context).unwrap_err();
            assert_eq!(error.code(), 400);
            assert!(error.tag().contains(context));
            assert!(error.tag().contains("u32"));
        }

        let decode = checked_db_u64(-1_i64, "member.uid").unwrap_err();
        assert!(matches!(decode, sqlx::Error::Decode(_)));
        assert!(decode.to_string().contains("member.uid"));
        let api_error = ApiError::from(decode);
        assert_eq!(api_error.code(), 500);

        let decode = checked_db_i32(u64::from(u32::MAX), "category.id").unwrap_err();
        assert!(matches!(decode, sqlx::Error::Decode(_)));
        assert!(decode.to_string().contains("category.id"));
    }

    #[test]
    fn count_policies_are_explicit() {
        assert_eq!(nonnegative_count(-1), 0);
        assert_eq!(
            nonnegative_count(i64::MAX),
            u64::try_from(i64::MAX).unwrap()
        );
        assert_eq!(saturating_count_u32(-1), 0);
        assert_eq!(saturating_count_u32(i64::MAX), u32::MAX);
        assert_eq!(saturating_count_i32(u64::MAX), i32::MAX);
    }

    #[test]
    fn float_conversion_rejects_invalid_values() {
        assert!(finite_f64_to_i64(f64::NAN, FloatRounding::Round, "distance").is_err());
        assert!(finite_f64_to_i64(f64::INFINITY, FloatRounding::Round, "distance").is_err());
        assert!(finite_f64_to_i64(f64::NEG_INFINITY, FloatRounding::Round, "distance").is_err());
        assert!(finite_f64_to_i64(f64::MAX, FloatRounding::Round, "distance").is_err());
        assert_eq!(
            finite_f64_to_i64(12.6, FloatRounding::Round, "distance").unwrap(),
            13
        );
        assert_eq!(
            finite_f64_to_i64(12.6, FloatRounding::Truncate, "points").unwrap(),
            12
        );
        assert!(integral_f64_to_u32(1.5, "points").is_err());
        assert_eq!(integral_f64_to_u32(12.0, "points").unwrap(), 12);
    }

    #[test]
    fn duration_millis_saturates_at_u64_max() {
        assert_eq!(saturating_millis(Duration::from_millis(42)), 42);
        assert_eq!(saturating_millis(Duration::MAX), u64::MAX);
    }

    #[test]
    fn database_float_errors_are_decode_errors() {
        let error = finite_to_f64_db(f64::NAN, "package.price").unwrap_err();
        assert!(matches!(error, sqlx::Error::Decode(_)));
        assert!(error.to_string().contains("package.price"));

        let error = finite_f64_to_i64_db(f64::INFINITY, FloatRounding::Round, "distance.meters")
            .unwrap_err();
        assert!(matches!(error, sqlx::Error::Decode(_)));

        let error = integral_f64_to_u32_db(1.5, "integral.order_price").unwrap_err();
        assert!(matches!(error, sqlx::Error::Decode(_)));
    }
}
