use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArithmicoParser;

#[derive(PartialEq, Debug)]
pub enum Node {
    Number { value: f64 },
    Symbol { name: String },
    Boolean { value: bool },
    Sum {
        values: Vec<Node>
    }
}

pub fn parse(input: &str) -> Node {
    let pairs = ArithmicoParser::parse(Rule::program, input)
        .expect("failed to parse")
        .next()
        .unwrap();

    transform(pairs)
}

fn transform(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::statement => {
            transform(pair.into_inner().next().unwrap())
        }
        Rule::number => {
            Node::Number { value: pair.as_str().parse().unwrap() }
        }
        Rule::boolean => {
            if pair.as_str() == "true" {
                Node::Boolean { value: true }
            } else {
                Node::Boolean { value: false }
            }
        }
        Rule::symbol => {
            Node::Symbol { name: pair.as_str().to_string() }
        }
        Rule::sum => {
            Node::Sum { 
                values: pair.into_inner().map(|item| transform(item)).collect() 
            }
        }
        _ => unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_float() {
        let result = parse("2.1");
        assert_eq!(result, Node::Number { value: 2.1 });
    }

    #[test]
    fn parse_true() {
        let result = parse("true");
        assert_eq!(result, Node::Boolean { value: true });
    }

    #[test]
    fn parse_false() {
        let result = parse("false");
        assert_eq!(result, Node::Boolean { value: false });
    }

    #[test]
    fn parse_symbol() {
        let result = parse("hello");
        assert_eq!(result, Node::Symbol { name: String::from("hello") });
    }

    #[test]
    fn parse_sum() {
        let result = parse("1 + 2 + 3");
        assert_eq!(
            result,
            Node::Sum { values: vec![
                Node::Number { value: 1.0 },
                Node::Number { value: 2.0 },
                Node::Number { value: 3.0 },
            ] }
        );
    }
}
