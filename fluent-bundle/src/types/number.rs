use std::borrow::Cow;
use std::convert::TryInto;
use std::default::Default;
use std::str::FromStr;

use fixed_decimal::{DoublePrecision, FixedDecimal};
use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::{DecimalError, FixedDecimalFormatter};
use icu_locid::{LanguageIdentifier as IcuLanguageIdentifier, ParserError};
use icu_provider::prelude::*;
use intl_memoizer::Memoizable;
use intl_pluralrules::operands::PluralOperands;
use unic_langid::LanguageIdentifier;

use crate::args::FluentArgs;
use crate::bundle::{FluentBundle, IcuDataProvider};
use crate::memoizer::MemoizerKind;
use crate::types::FluentValue;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FluentNumberType {
    #[default]
    Cardinal,
    Ordinal,
}

impl From<&str> for FluentNumberType {
    fn from(input: &str) -> Self {
        match input {
            "cardinal" => Self::Cardinal,
            "ordinal" => Self::Ordinal,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq)]
pub enum FluentNumberStyle {
    #[default]
    Decimal,
    Currency,
    Percent,
}

impl From<&str> for FluentNumberStyle {
    fn from(input: &str) -> Self {
        match input {
            "decimal" => Self::Decimal,
            "currency" => Self::Currency,
            "percent" => Self::Percent,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq)]
pub enum FluentNumberCurrencyDisplayStyle {
    #[default]
    Symbol,
    Code,
    Name,
}

impl From<&str> for FluentNumberCurrencyDisplayStyle {
    fn from(input: &str) -> Self {
        match input {
            "symbol" => Self::Symbol,
            "code" => Self::Code,
            "name" => Self::Name,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FluentNumberUseGrouping {
    Auto,
    False,
    Always,
    Min2,
}

impl Default for FluentNumberUseGrouping {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct FluentNumberOptions {
    pub r#type: FluentNumberType,
    pub style: FluentNumberStyle,
    pub currency: Option<String>,
    pub currency_display: FluentNumberCurrencyDisplayStyle,
    pub use_grouping: FluentNumberUseGrouping,
    pub minimum_integer_digits: Option<usize>,
    pub minimum_fraction_digits: Option<usize>,
    pub maximum_fraction_digits: Option<usize>,
    pub minimum_significant_digits: Option<usize>,
    pub maximum_significant_digits: Option<usize>,
}

impl FluentNumberOptions {
    pub fn merge(&mut self, opts: &FluentArgs) {
        for (key, value) in opts.iter() {
            match (key, value) {
                ("type", FluentValue::String(n)) => {
                    self.r#type = n.as_ref().into();
                }
                ("style", FluentValue::String(n)) => {
                    self.style = n.as_ref().into();
                }
                ("currency", FluentValue::String(n)) => {
                    self.currency = Some(n.to_string());
                }
                ("currencyDisplay", FluentValue::String(n)) => {
                    self.currency_display = n.as_ref().into();
                }
                ("useGrouping", FluentValue::String(n)) => {
                    self.use_grouping = match n.as_ref() {
                        "false" => FluentNumberUseGrouping::False,
                        "always" => FluentNumberUseGrouping::Always,
                        "min2" => FluentNumberUseGrouping::Min2,
                        _ => FluentNumberUseGrouping::Auto,
                    }
                }
                ("minimumIntegerDigits", FluentValue::Number(n)) => {
                    self.minimum_integer_digits = Some(n.into());
                }
                ("minimumFractionDigits", FluentValue::Number(n)) => {
                    self.minimum_fraction_digits = Some(n.into());
                }
                ("maximumFractionDigits", FluentValue::Number(n)) => {
                    self.maximum_fraction_digits = Some(n.into());
                }
                ("minimumSignificantDigits", FluentValue::Number(n)) => {
                    self.minimum_significant_digits = Some(n.into());
                }
                ("maximumSignificantDigits", FluentValue::Number(n)) => {
                    self.maximum_significant_digits = Some(n.into());
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FluentNumber {
    pub value: f64,
    pub options: FluentNumberOptions,
}

impl FluentNumber {
    pub const fn new(value: f64, options: FluentNumberOptions) -> Self {
        Self { value, options }
    }

    pub fn as_string<R, M: MemoizerKind>(&self, bundle: &FluentBundle<R, M>) -> Cow<'static, str> {
        let fixed_decimal = self.as_fixed_decimal();
        let options = FormatterOptions {
            use_grouping: self.options.use_grouping,
        };
        if let Some(data_provider) = &bundle.icu_data_provider {
            let formatted = bundle
                .intls
                .with_try_get_threadsafe::<NumberFormatter, _, _>(
                    (options,),
                    data_provider,
                    |formatter| formatter.0.format_to_string(&fixed_decimal),
                )
                .unwrap();
            return formatted.into();
        }

        fixed_decimal.to_string().into()
    }

    fn as_fixed_decimal(&self) -> FixedDecimal {
        let precision = if let Some(maxsd) = self.options.maximum_significant_digits {
            DoublePrecision::SignificantDigits(maxsd as u8)
        } else {
            DoublePrecision::Floating
        };

        let mut fixed_decimal = FixedDecimal::try_from_f64(self.value, precision).unwrap();

        if let Some(minfd) = self.options.minimum_fraction_digits {
            fixed_decimal.pad_end(-(minfd as i16));
        }
        if let Some(minid) = self.options.minimum_integer_digits {
            fixed_decimal.pad_start(minid as i16);
        }
        fixed_decimal
    }

    pub fn as_string_basic(&self) -> String {
        self.as_fixed_decimal().to_string()
    }
}

impl FromStr for FluentNumber {
    type Err = std::num::ParseFloatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        f64::from_str(input).map(|n| {
            let mfd = input.find('.').map(|pos| input.len() - pos - 1);
            let opts = FluentNumberOptions {
                minimum_fraction_digits: mfd,
                ..Default::default()
            };
            Self::new(n, opts)
        })
    }
}

impl<'l> From<FluentNumber> for FluentValue<'l> {
    fn from(input: FluentNumber) -> Self {
        FluentValue::Number(input)
    }
}

macro_rules! from_num {
    ($num:ty) => {
        impl From<$num> for FluentNumber {
            fn from(n: $num) -> Self {
                Self {
                    value: n as f64,
                    options: FluentNumberOptions::default(),
                }
            }
        }
        impl From<&$num> for FluentNumber {
            fn from(n: &$num) -> Self {
                Self {
                    value: *n as f64,
                    options: FluentNumberOptions::default(),
                }
            }
        }
        impl From<FluentNumber> for $num {
            fn from(input: FluentNumber) -> Self {
                input.value as $num
            }
        }
        impl From<&FluentNumber> for $num {
            fn from(input: &FluentNumber) -> Self {
                input.value as $num
            }
        }
        impl From<$num> for FluentValue<'_> {
            fn from(n: $num) -> Self {
                FluentValue::Number(n.into())
            }
        }
        impl From<&$num> for FluentValue<'_> {
            fn from(n: &$num) -> Self {
                FluentValue::Number(n.into())
            }
        }
    };
    ($($num:ty)+) => {
        $(from_num!($num);)+
    };
}

impl From<&FluentNumber> for PluralOperands {
    fn from(input: &FluentNumber) -> Self {
        let mut operands: Self = input
            .value
            .try_into()
            .expect("Failed to generate operands out of FluentNumber");
        if let Some(mfd) = input.options.minimum_fraction_digits {
            if mfd > operands.v {
                operands.f *= 10_u64.pow(mfd as u32 - operands.v as u32);
                operands.v = mfd;
            }
        }
        // XXX: Add support for other options.
        operands
    }
}

from_num!(i8 i16 i32 i64 i128 isize);
from_num!(u8 u16 u32 u64 u128 usize);
from_num!(f32 f64);

pub type NumberFormatProvider = Box<dyn DataProvider<DecimalSymbolsV1Marker>>;

#[derive(Clone, Hash, PartialEq, Eq)]
struct FormatterOptions {
    use_grouping: FluentNumberUseGrouping,
}

struct NumberFormatter(FixedDecimalFormatter);

#[derive(Debug)]
#[allow(dead_code)]
enum NumberFormatterError {
    ParserError(ParserError),
    DecimalError(DecimalError),
}

impl Memoizable for NumberFormatter {
    type Args = (FormatterOptions,);
    type Error = NumberFormatterError;
    type DataProvider = IcuDataProvider;

    fn construct(
        lang: LanguageIdentifier,
        args: Self::Args,
        data_provider: &Self::DataProvider,
    ) -> Result<Self, Self::Error> {
        let locale = to_icu_lang_id(lang).map_err(NumberFormatterError::ParserError)?;

        let mut options: FixedDecimalFormatterOptions = Default::default();
        options.grouping_strategy = match args.0.use_grouping {
            FluentNumberUseGrouping::Auto => GroupingStrategy::Auto,
            FluentNumberUseGrouping::False => GroupingStrategy::Never,
            FluentNumberUseGrouping::Always => GroupingStrategy::Always,
            FluentNumberUseGrouping::Min2 => GroupingStrategy::Min2,
        };

        let inner = FixedDecimalFormatter::try_new_with_any_provider(
            data_provider,
            &locale.into(),
            options,
        )
        .map_err(NumberFormatterError::DecimalError)?;
        Ok(NumberFormatter(inner))
    }
}

fn to_icu_lang_id(lang: LanguageIdentifier) -> Result<IcuLanguageIdentifier, ParserError> {
    return IcuLanguageIdentifier::try_from_locale_bytes(lang.to_string().as_bytes());
}

#[cfg(test)]
mod tests {
    use super::to_icu_lang_id;
    use unic_langid::langid;

    use crate::types::FluentValue;

    #[test]
    fn value_from_copy_ref() {
        let x = 1i16;
        let y = &x;
        let z: FluentValue = y.into();
        assert_eq!(z, FluentValue::try_number("1"));
    }

    #[test]
    fn lang_to_icu() {
        assert_eq!(
            to_icu_lang_id(langid!("en-US")).unwrap(),
            icu_locid::langid!("en-US")
        );
        assert_eq!(
            to_icu_lang_id(langid!("pl")).unwrap(),
            icu_locid::langid!("pl")
        );
    }
}
