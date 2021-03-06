use crate::{Excess, ExcessHandler, ExcessHandlingFunction, PadDirection, Width};
use core::fmt::{Display, Error, Formatter};

#[cfg(feature = "std")]
use derive_builder::Builder;

/// Pad a single value.
///
/// **Key traits:**
/// * [`Display`]: Displays the padded version of the value.
///
/// **Example:** Pad dash characters to the left of a string
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedValue, PadDirection, ForbidExcess};
/// let padded_value = PaddedValue {
///     value: "abcdef",
///     pad_block: '-',
///     total_width: 9,
///     pad_direction: PadDirection::Left,
///     handle_excess: ForbidExcess,
/// };
/// assert_eq!(padded_value.to_string(), "---abcdef");
/// ```
///
/// **Example:** Use a [builder](PaddedValueBuilder) _(requires `std` feature)_
///
/// ```
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedValueBuilder, PadDirection, ForbidExcess};
/// let padded_value = PaddedValueBuilder::default()
///     .value("abcdef")
///     .pad_block('-')
///     .total_width(9)
///     .pad_direction(PadDirection::Left)
///     .handle_excess(ForbidExcess)
///     .build()
///     .unwrap();
/// assert_eq!(padded_value.to_string(), "---abcdef");
/// # }
/// # #[cfg(not(feature = "std"))] fn main() {}
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Builder))]
pub struct PaddedValue<
    Value,
    PadBlock = char,
    HandleExcess = ExcessHandlingFunction<Value, PadBlock>,
> where
    Value: Width,
    PadBlock: Display,
    HandleExcess: ExcessHandler<Value, PadBlock>,
{
    /// Value to be padded.
    pub value: Value,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Total width to fulfill.
    pub total_width: usize,
    /// Where to place the pad.
    pub pad_direction: PadDirection,
    /// How to write when the actual width of `value` exceeds `total_width`.
    pub handle_excess: HandleExcess,
}

impl<Value, PadBlock, HandleExcess> Display for PaddedValue<Value, PadBlock, HandleExcess>
where
    Value: Width,
    PadBlock: Display,
    HandleExcess: ExcessHandler<Value, PadBlock>,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let PaddedValue {
            value,
            pad_block,
            total_width,
            pad_direction,
            handle_excess,
        } = self;
        let total_width = *total_width;
        let value_width = value.width();
        let pad_width = if total_width >= value_width {
            total_width - value_width
        } else {
            return handle_excess.handle_excess(
                Excess {
                    value,
                    value_width,
                    total_width,
                    pad_block,
                },
                formatter,
            );
        };
        let pad = fmt_iter::repeat(pad_block, pad_width);
        match *pad_direction {
            PadDirection::Left => write!(formatter, "{}{}", pad, value),
            PadDirection::Right => write!(formatter, "{}{}", value, pad),
        }
    }
}
