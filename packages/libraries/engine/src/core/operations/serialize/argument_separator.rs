use crate::DecimalFormat;

use super::SerializeNodeOptions;

pub(crate) fn get_argument_separator(options: &SerializeNodeOptions) -> String {
    match options.decimal_format {
        DecimalFormat::Comma => String::from("; "),
        DecimalFormat::Dot => String::from(", "),
    }
}
