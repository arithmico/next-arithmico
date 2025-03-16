use crate::core::{Context, DecimalFormat};

pub fn get_decimal_separator(context: &Context) -> String {
    match context.decimal_format {
        DecimalFormat::Comma => String::from(","),
        DecimalFormat::Dot => String::from("."),
    }
}
