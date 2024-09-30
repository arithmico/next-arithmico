use crate::SerializeNodeOptions;

pub(crate) fn get_argument_separator(options: &SerializeNodeOptions) -> String {
    match options.decimal_format {
        common::DecimalFormat::Comma => String::from("; "),
        common::DecimalFormat::Dot => String::from(", "),
    }
}
