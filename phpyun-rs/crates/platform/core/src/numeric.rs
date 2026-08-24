//! Explicit numeric conversion helpers.
//!
//! Rust's `as` conversions silently truncate, wrap, or saturate.  These helpers
//! keep the conversion policy visible and attach enough context to diagnose a
//! bad request or a corrupt database value without reproducing it locally.

use std::any::type_name;
use std::fmt::{self, Display};
use std::time::Duration;

use num_traits::ToPrimitive;

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

pub fn finite_to_f64<T>(value: T, context: &'static str) -> AppResult<f64>
where
    T: Copy + Display + ToPrimitive,
{
    value.to_f64().ok_or_else(|| {
        ApiError::internal(NumericConversionError::new::<f64>(
            context,
            value,
            "precision/range conversion failed",
        ))
    })
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
    if !value.is_finite() {
        return Err(ApiError::internal(NumericConversionError::new::<i64>(
            context,
            value,
            "value is not finite",
        )));
    }
    let normalized = match rounding {
        FloatRounding::Round => value.round(),
        FloatRounding::Truncate => value.trunc(),
    };
    normalized.to_i64().ok_or_else(|| {
        ApiError::internal(NumericConversionError::new::<i64>(
            context,
            value,
            "value is outside the i64 range",
        ))
    })
}

pub fn integral_f64_to_u32(value: f64, context: &'static str) -> AppResult<u32> {
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(ApiError::internal(NumericConversionError::new::<u32>(
            context,
            value,
            "value must be a finite whole number",
        )));
    }
    value.to_u32().ok_or_else(|| {
        ApiError::internal(NumericConversionError::new::<u32>(
            context,
            value,
            "value is outside the u32 range",
        ))
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
    }
}
