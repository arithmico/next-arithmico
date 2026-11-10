use std::collections::BTreeSet;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::solve_linear_equation;
use node::{
    Division, DowncastNodeVec, Equals, GetNodeType, Negate, Node, Number,
    Product, Sum, Symbol, Tensor,
};

struct LinearExpression {
    coefficients: Vec<f64>,
    constant: f64,
}

fn extract_linear_expression(
    node: &Node,
    variables: &[&str],
) -> Result<LinearExpression, Error> {
    if let Ok(number) = <&Number>::try_from(node) {
        return Ok(LinearExpression {
            coefficients: vec![0.0; variables.len()],
            constant: number.value,
        });
    }

    if let Ok(symbol) = <&Symbol>::try_from(node) {
        let mut coefficients = vec![0.0; variables.len()];

        let index = variables
            .iter()
            .position(|variable| *variable == symbol.name)
            .ok_or_else(|| Error::invalid_node(node.node_type()))?;

        coefficients[index] = 1.0;

        return Ok(LinearExpression {
            coefficients,
            constant: 0.0,
        });
    }

    if let Ok(negate) = <&Negate>::try_from(node) {
        let mut expression =
            extract_linear_expression(&negate.value, variables)?;

        for coefficient in &mut expression.coefficients {
            *coefficient = -*coefficient;
        }

        expression.constant = -expression.constant;

        return Ok(expression);
    }

    if let Ok(sum) = <&Sum>::try_from(node) {
        let mut result = LinearExpression {
            coefficients: vec![0.0; variables.len()],
            constant: 0.0,
        };

        for summand in &sum.elements {
            let expression = extract_linear_expression(summand, variables)?;

            for (result, coefficient) in
                result.coefficients.iter_mut().zip(expression.coefficients)
            {
                *result += coefficient;
            }

            result.constant += expression.constant;
        }

        return Ok(result);
    }

    if let Ok(product) = <&Product>::try_from(node) {
        let mut constant = 1.0;
        let mut variable_expression = None;

        for factor in &product.elements {
            let expression = extract_linear_expression(factor, variables)?;

            let is_constant = expression
                .coefficients
                .iter()
                .all(|coefficient| *coefficient == 0.0);

            if is_constant {
                constant *= expression.constant;
            } else if variable_expression.is_some() {
                return Err(Error::non_linear_expression());
            } else {
                variable_expression = Some(expression);
            }
        }

        if let Some(mut expression) = variable_expression {
            for coefficient in &mut expression.coefficients {
                *coefficient *= constant;
            }

            expression.constant *= constant;

            return Ok(expression);
        }

        return Ok(LinearExpression {
            coefficients: vec![0.0; variables.len()],
            constant,
        });
    }

    if let Ok(division) = <&Division>::try_from(node) {
        let mut dividend =
            extract_linear_expression(&division.dividend, variables)?;
        let divisor = extract_linear_expression(&division.divisor, variables)?;

        if divisor
            .coefficients
            .iter()
            .any(|coefficient| *coefficient != 0.0)
        {
            return Err(Error::non_linear_expression());
        }

        if divisor.constant == 0.0 {
            return Err(Error::non_linear_expression());
        }

        for coefficient in &mut dividend.coefficients {
            *coefficient /= divisor.constant;
        }

        dividend.constant /= divisor.constant;

        return Ok(dividend);
    }

    Err(Error::non_linear_expression())
}

#[derive(FunctionArguments)]
#[name("lsolve")]
#[description(Language::German, "Löst das lineare Gleichungssystem.")]
#[description(Language::English, "Solves the linear equation system.")]
pub struct LSolveArgs<'a> {
    #[description(Language::German, "Lineare Gleichungen")]
    #[description(Language::English, "linear equations")]
    equations: &'a Tensor,
}

pub struct LSolveEndpoint;

impl FunctionEndpoint for LSolveEndpoint {
    type Output = Tensor;
    type Arguments<'a> = LSolveArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LSolveArgs { equations }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        let equations = equations
            .downcast::<Equals>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .into_iter()
            .collect::<Vec<_>>();

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

        for equation in equations {
            let left = extract_linear_expression(&equation.left, &variables)?;
            let right = extract_linear_expression(&equation.right, &variables)?;

            coefficients.extend(
                left.coefficients
                    .iter()
                    .zip(&right.coefficients)
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
