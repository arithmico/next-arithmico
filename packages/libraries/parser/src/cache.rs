use std::{cell::RefCell, collections::HashMap};

use ast::Node;

use crate::{error::ParseNodeError, ParseResult};

type CachedParseResult = Result<(String, Node), nom::Err<ParseNodeError>>;

#[derive(Clone, Debug)]
struct Cache {
    entries: RefCell<HashMap<String, HashMap<String, CachedParseResult>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            entries: RefCell::new(HashMap::new()),
        }
    }

    pub fn set(&self, table: &str, input: &str, output: ParseResult) {
        let mut entries = self.entries.borrow_mut();
        let table = match entries.get_mut(table) {
            Some(table) => table,
            None => {
                entries.insert(table.to_string(), HashMap::new());
                entries.get_mut(table).unwrap()
            }
        };

        table.insert(
            input.to_string(),
            match output {
                Ok((remaining_input, node)) => {
                    Ok((remaining_input.to_string(), node))
                }
                Err(err) => Err(err),
            },
        );
    }

    pub fn get(&self, table: &str, input: &str) -> Option<CachedParseResult> {
        let entries = self.entries.borrow();

        let entry = entries
            .get(table)
            .map(|table| table.get(input))
            .unwrap_or(None);

        entry.cloned()
    }

    pub fn with<'a>(
        &self,
        table: &'a str,
        input: &'a str,
        f: impl Fn(&'a str) -> ParseResult<'a>,
    ) -> ParseResult<'a> {
        match self.get(table, input) {
            Some(result) => match result {
                Ok((remaining_input, node)) => {
                    let start_index = input.len() - remaining_input.len();
                    Ok((&input[start_index..], node))
                }
                Err(err) => Err(err),
            },
            _ => {
                let result = f(input);
                self.set(table, input, result.clone());
                result
            }
        }
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
    }
}

pub fn with_cache<'a>(
    parser_id: &'static str,
    f: impl Fn(&str) -> ParseResult + 'static + Copy,
) -> impl Fn(&str) -> ParseResult {
    move |input| {
        let result = PARSER_CACHE.with(|cache| cache.with(parser_id, input, f));
        result
    }
}

thread_local! {
    static PARSER_CACHE: Cache = Cache::new();
}
