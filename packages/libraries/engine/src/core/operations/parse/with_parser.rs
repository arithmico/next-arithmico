use super::{with_cache, with_input_len, ParseResult};

pub fn with_parser<'a>(
    parser_id: &'static str,
    f: impl Fn(&str) -> ParseResult + 'static + Copy,
) -> impl Fn(&str) -> ParseResult {
    with_input_len(with_cache(parser_id, f))
}
