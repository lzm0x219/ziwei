//! JavaScript adapters for the `ziwei` core.
//!
//! The native build exports a napi-rs class for `@ziweijs/core`; the
//! `wasm32-unknown-unknown` build exports the equivalent wasm-bindgen class for
//! `@ziweijs/core-wasm`. Both adapters expose only `Ziwei.fromBirth()` and keep
//! the domain model in the `ziwei` crate.

#![deny(clippy::all)]

use ziwei::{Gender, Ziwei as CoreZiwei, ZiweiBirth as CoreZiweiBirth};

#[derive(Debug)]
#[cfg_attr(target_arch = "wasm32", derive(serde::Deserialize))]
struct BirthInput {
    gender: String,
    year: f64,
    month: f64,
    day: f64,
    hour: f64,
}

fn chart_from_birth(input: BirthInput) -> Result<CoreZiwei, String> {
    let gender = match input.gender.as_str() {
        "Yin" => Gender::Yin,
        "Yang" => Gender::Yang,
        value => return Err(format!("gender must be \"Yin\" or \"Yang\", got {value:?}")),
    };
    let year = integer_i32("year", input.year)?;
    let month = integer_u8("month", input.month)?;
    let day = integer_u8("day", input.day)?;
    let hour = integer_u8("hour", input.hour)?;
    let birth = CoreZiweiBirth::try_new(gender, year, month, day, hour)
        .map_err(|error| error.to_string())?;

    Ok(CoreZiwei::from_birth(birth))
}

fn integer_i32(field: &str, value: f64) -> Result<i32, String> {
    if !value.is_finite()
        || value.fract() != 0.0
        || value < f64::from(i32::MIN)
        || value > f64::from(i32::MAX)
    {
        return Err(format!(
            "{field} must be a finite integer within {}..={}, got {value}",
            i32::MIN,
            i32::MAX
        ));
    }

    Ok(value as i32)
}

fn integer_u8(field: &str, value: f64) -> Result<u8, String> {
    if !value.is_finite()
        || value.fract() != 0.0
        || value < f64::from(u8::MIN)
        || value > f64::from(u8::MAX)
    {
        return Err(format!(
            "{field} must be a finite integer within {}..={}, got {value}",
            u8::MIN,
            u8::MAX
        ));
    }

    Ok(value as u8)
}

#[cfg(not(target_arch = "wasm32"))]
mod node {
    use super::{BirthInput, chart_from_birth};
    use napi::{Error, Status, bindgen_prelude::Result};
    use napi_derive::napi;
    use ziwei::Ziwei as CoreZiwei;

    /// Plain JavaScript representation of the core `ZiweiBirth` fields.
    #[napi(object, object_to_js = false)]
    pub struct NapiZiweiBirth {
        /// `Yin` or `Yang`.
        pub gender: String,
        /// Lunar year number.
        pub year: f64,
        /// Lunar month index, `0..=11`.
        pub month: f64,
        /// Lunar day, `1..=30`.
        pub day: f64,
        /// Hour-branch index, `0..=11`.
        pub hour: f64,
    }

    impl From<NapiZiweiBirth> for BirthInput {
        fn from(value: NapiZiweiBirth) -> Self {
            Self {
                gender: value.gender,
                year: value.year,
                month: value.month,
                day: value.day,
                hour: value.hour,
            }
        }
    }

    /// Native handle owned by the public JavaScript `Ziwei` facade.
    #[napi]
    pub struct NativeZiwei {
        _inner: CoreZiwei,
    }

    #[napi]
    impl NativeZiwei {
        /// Construct a chart from validated lunar birth fields.
        #[napi(factory)]
        pub fn from_birth(birth: NapiZiweiBirth) -> Result<Self> {
            chart_from_birth(birth.into())
                .map(|inner| Self { _inner: inner })
                .map_err(|message| Error::new(Status::InvalidArg, message))
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::{BirthInput, chart_from_birth};
    use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};
    use ziwei::Ziwei as CoreZiwei;

    /// WebAssembly handle owned by the public JavaScript `Ziwei` facade.
    #[wasm_bindgen]
    pub struct NativeZiwei {
        _inner: CoreZiwei,
    }

    #[wasm_bindgen]
    impl NativeZiwei {
        /// Construct a chart from validated lunar birth fields.
        #[wasm_bindgen(js_name = fromBirth)]
        pub fn from_birth(birth: JsValue) -> Result<NativeZiwei, JsError> {
            let birth: BirthInput = serde_wasm_bindgen::from_value(birth)
                .map_err(|_| JsError::new("invalid ZiweiBirth object"))?;

            chart_from_birth(birth)
                .map(|inner| Self { _inner: inner })
                .map_err(|message| JsError::new(&message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_birth() -> BirthInput {
        BirthInput {
            gender: "Yang".to_owned(),
            year: 1984.0,
            month: 2.0,
            day: 1.0,
            hour: 4.0,
        }
    }

    #[test]
    fn from_birth_preserves_the_real_lunar_year() {
        let chart = chart_from_birth(valid_birth()).expect("valid birth should build a chart");

        assert_eq!(chart.birth_year(), Some(1984));
    }

    #[test]
    fn invalid_gender_is_rejected_before_entering_the_core() {
        let mut birth = valid_birth();
        birth.gender = "Male".to_owned();

        assert_eq!(
            chart_from_birth(birth).expect_err("invalid gender should fail"),
            "gender must be \"Yin\" or \"Yang\", got \"Male\""
        );
    }

    #[test]
    fn numeric_representation_is_checked_before_core_validation() {
        let mut birth = valid_birth();
        birth.year = f64::NAN;

        assert!(
            chart_from_birth(birth)
                .expect_err("NaN year should fail")
                .starts_with("year must be a finite integer")
        );
    }
}
