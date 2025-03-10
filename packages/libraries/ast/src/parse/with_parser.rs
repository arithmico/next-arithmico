use super::{cache::with_cache, node::ParseResult, trace::with_input_len};

pub fn with_parser<'a>(
    parser_id: &'static str,
    f: impl Fn(&str) -> ParseResult + 'static + Copy,
) -> impl Fn(&str) -> ParseResult {
    with_input_len(with_cache(parser_id, f))
}
