use ast::DecimalFormat;

use crate::SerializeNodeOptions;

pub(crate) fn get_decimal_separator(options: &SerializeNodeOptions) -> String {
    match options.decimal_format {
        DecimalFormat::Comma => String::from(","),
        DecimalFormat::Dot => String::from("."),
    }
}
