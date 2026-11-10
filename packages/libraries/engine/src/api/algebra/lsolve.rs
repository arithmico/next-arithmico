use std::collections::BTreeSet;

use algebra::PartialEvaluate;
use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::solve_linear_equation;
use node::{Equals, GetNodeType, Node, Number, Symbol, Tensor};

#[derive(Debug, PartialEq)]
struct LinearExpression {
    coefficients: Vec<f64>,
    constant: f64,
}

impl LinearExpression {
    fn zero(variable_count: usize) -> Self {
        Self {
            coefficients: vec![0.0; variable_count],
            constant: 0.0,
        }
    }

    fn is_constant(&self) -> bool {
        self.coefficients
            .iter()
            .all(|coefficient| *coefficient == 0.0)
    }

    fn scale(&mut self, factor: f64) {
        for coefficient in &mut self.coefficients {
            *coefficient *= factor;
        }

        self.constant *= factor;
    }

    fn add(&mut self, expression: LinearExpression) {
        for (coefficient, other) in
            self.coefficients.iter_mut().zip(expression.coefficients)
        {
            *coefficient += other;
        }

        self.constant += expression.constant;
    }
}

fn extract_linear_expression(
    node: &Node,
    variables: &[&str],
) -> Result<LinearExpression, Error> {
    match node {
        Node::Number(number) => {
            let mut expression = LinearExpression::zero(variables.len());
            expression.constant = number.value;

            Ok(expression)
        }

        Node::Symbol(symbol) => {
            let index = variables
                .iter()
                .position(|variable| *variable == symbol.name)
                .ok_or_else(|| Error::invalid_node(node.node_type()))?;

            let mut expression = LinearExpression::zero(variables.len());
            expression.coefficients[index] = 1.0;

            Ok(expression)
        }

        Node::Negate(negate) => {
            let mut expression =
                extract_linear_expression(&negate.value, variables)?;

            expression.scale(-1.0);

            Ok(expression)
        }

        Node::Sum(sum) => {
            let mut result = LinearExpression::zero(variables.len());

            for summand in &sum.elements {
                result.add(extract_linear_expression(summand, variables)?);
            }

            Ok(result)
        }

        Node::Product(product) => {
            let mut constant = 1.0;
            let mut variable_expression = None;

            for factor in &product.elements {
                let expression = extract_linear_expression(factor, variables)?;

                if expression.is_constant() {
                    constant *= expression.constant;
                } else if variable_expression.is_some() {
                    return Err(Error::non_linear_expression());
                } else {
                    variable_expression = Some(expression);
                }
            }

            if let Some(mut expression) = variable_expression {
                expression.scale(constant);

                Ok(expression)
            } else {
                let mut expression = LinearExpression::zero(variables.len());
                expression.constant = constant;

                Ok(expression)
            }
        }

        Node::Division(division) => {
            let mut dividend =
                extract_linear_expression(&division.dividend, variables)?;

            let divisor =
                extract_linear_expression(&division.divisor, variables)?;

            if !divisor.is_constant() || divisor.constant == 0.0 {
                return Err(Error::non_linear_expression());
            }

            dividend.scale(1.0 / divisor.constant);

            Ok(dividend)
        }

        _ => Err(Error::non_linear_expression()),
    }
}

#[derive(FunctionArguments)]
#[name("lsolve")]
#[description(Language::German, "Löst das lineare Gleichungssystem.")]
#[description(Language::English, "Solves the linear equation system.")]
pub struct LSolveArgs<'a> {
    #[skip_evaluate]
    #[description(Language::German, "Lineare Gleichungen")]
    #[description(Language::English, "linear equations")]
    equations: Vec<&'a Equals>,
}

pub struct LSolveEndpoint;

impl FunctionEndpoint for LSolveEndpoint {
    type Output = Tensor;
    type Arguments<'a> = LSolveArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LSolveArgs { equations }: Self::Arguments<'a>,
        options: Options,
    ) -> Result<Self::Output, Error> {
        let mut equations = equations.into_iter().cloned().collect::<Vec<_>>();

        for equation in &mut equations {
            equation.left.partial_evaluate(options)?;
            equation.right.partial_evaluate(options)?;
        }

        let variables = equations
            .iter()
            .flat_map(|equation| {
                equation
                    .left
                    .get_symbol_names()
                    .into_iter()
                    .chain(equation.right.get_symbol_names())
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if equations.len() != variables.len() {
            return Err(Error::incompatible_vector_dimensions(
                equations.len(),
                variables.len(),
            ));
        }

        let mut coefficients =
            Vec::with_capacity(equations.len() * variables.len());
        let mut constants = Vec::with_capacity(equations.len());

        for equation in &equations {
            let left = extract_linear_expression(&equation.left, &variables)?;
            let right = extract_linear_expression(&equation.right, &variables)?;

            coefficients.extend(
                left.coefficients
                    .into_iter()
                    .zip(right.coefficients)
                    .map(|(left, right)| left - right),
            );

            constants.push(right.constant - left.constant);
        }

        let solution =
            solve_linear_equation(variables.len(), &coefficients, &constants)
                .map_to_error_kind(ErrorKind::RuntimeError)?;

        let result = variables
            .into_iter()
            .zip(solution)
            .map(|(variable, value)| {
                Equals::new(Symbol::new(variable), Number::new_node(value))
            })
            .collect::<Vec<_>>();

        Ok(Tensor::new(result))
    }
}

#[cfg(test)]
mod tests {
    use node::{Division, Negate, Number, Product, Sum, Symbol};

    use super::*;

    #[test]
    fn extract_number() {
        let node = Number::new_node(5.0);
        let result = extract_linear_expression(&node, &["x", "y"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![0.0, 0.0],
                constant: 5.0,
            }
        );
    }

    #[test]
    fn extract_symbol() {
        let node = Symbol::new("y");
        let result = extract_linear_expression(&node, &["x", "y"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![0.0, 1.0],
                constant: 0.0,
            }
        );
    }

    #[test]
    fn extract_sum() {
        let node = Sum::new(vec![
            Symbol::new("x"),
            Symbol::new("y"),
            Number::new_node(3.0),
        ]);
        let result = extract_linear_expression(&node, &["x", "y"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![1.0, 1.0],
                constant: 3.0,
            }
        );
    }

    #[test]
    fn extract_negate() {
        let node = Negate::new(Sum::new(vec![
            Symbol::new("x"),
            Number::new_node(2.0),
        ]));
        let result = extract_linear_expression(&node, &["x"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![-1.0],
                constant: -2.0,
            }
        );
    }

    #[test]
    fn extract_product() {
        let node = Product::new(vec![
            Number::new_node(3.0),
            Sum::new(vec![Symbol::new("x"), Number::new_node(2.0)]),
        ]);
        let result = extract_linear_expression(&node, &["x"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![3.0],
                constant: 6.0,
            }
        );
    }

    #[test]
    fn extract_multiple_constant_factors() {
        let node = Product::new(vec![
            Number::new_node(2.0),
            Number::new_node(3.0),
            Symbol::new("x"),
        ]);
        let result = extract_linear_expression(&node, &["x"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![6.0],
                constant: 0.0,
            }
        );
    }

    #[test]
    fn extract_division() {
        let node = Division::new(
            Sum::new(vec![Number::new_node(2.0), Symbol::new("x")]),
            Number::new_node(2.0),
        );
        let result = extract_linear_expression(&node, &["x"]).unwrap();

        assert_eq!(
            result,
            LinearExpression {
                coefficients: vec![0.5],
                constant: 1.0,
            }
        );
    }

    #[test]
    fn reject_product_of_variables() {
        let node = Product::new(vec![Symbol::new("x"), Symbol::new("y")]);

        assert!(extract_linear_expression(&node, &["x", "y"]).is_err());
    }

    #[test]
    fn reject_variable_divisor() {
        let node = Division::new(Number::new_node(1.0), Symbol::new("x"));

        assert!(extract_linear_expression(&node, &["x"]).is_err());
    }

    #[test]
    fn reject_non_linear_node() {
        let node = node::Power::new(Symbol::new("x"), Number::new_node(2.0));

        assert!(extract_linear_expression(&node, &["x"]).is_err());
    }
}
