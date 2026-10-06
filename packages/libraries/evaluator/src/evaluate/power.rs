use node::{Node, Number, Power, Product, Tensor};
use trace::{Tracable, TracableMut};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Power {
    fn evaluate(&self, options: Options) -> Result<Node, Error> {
        let base = self.base.evaluate(options)?;
        let exponent = self.exponent.evaluate(options)?;

        match (&base, &exponent) {
            (Node::Number(base), Node::Number(exponent))
                if cfg!(feature = "operator_power_number_number") =>
            {
                if exponent.value == 0. {
                    return Ok(Number::new_node(1.));
                }

                if base.value == 0. {
                    return Ok(Number::new_node(0.));
                }

                Ok(Number::new_node(base.value.powf(exponent.value)))
            }
            (Node::Tensor(base), Node::Number(exponent))
                if cfg!(feature = "operator_power_matrix_number") =>
            {
                matrix_exponentiation(base, exponent, options)
            }
            _ => Err(Error::unsupported_operation()
                .with_optional_span(base.hull())
                .with_optional_span(exponent.hull())),
        }
    }
}

fn matrix_exponentiation(
    base: &Tensor,
    exponent: &Number,
    options: Options,
) -> Result<Node, Error> {
    let size =
        match base.shape.as_slice() {
            [n, m] if n == m => Ok(*n),
            _ => Err(Error::not_a_square_matrix()
                .with_optional_new_frame(base.hull())),
        }?;

    if exponent.value.fract() != 0.0 {
        return Err(Error::not_an_integer().with_optional_span(exponent.hull()));
    }

    let n = exponent.value.trunc() as i64;

    if n < 0 {
        return Err(Error::not_positive().with_optional_span(exponent.hull()));
    }

    let mut elements = Vec::with_capacity(size * size);
    for i in 0..size {
        for j in 0..size {
            if i == j {
                elements.push(Number::new(1.0).into());
            } else {
                elements.push(Number::new(0.0).into());
            }
        }
    }

    // exponentiation by squaring algorithm
    let mut result: Node =
        Tensor::new_with_shape(vec![size, size], elements).into();
    let mut a: Node = base.clone().into();
    let mut n = n as u64;

    while n > 0 {
        if n % 2 == 1 {
            result = Product::new(vec![result, a.clone()]).evaluate(options)?;
        }
        a = Product::new(vec![a.clone(), a]).evaluate(options)?;

        n /= 2;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{Api, Stack};

    use super::*;
    use lexer::Span;
    use trace::TracableMut;

    #[test]
    fn evaluate_power_number_number() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Power::new(Number::new_node(8.), Number::new_node(2.))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Number::new_node(64.));
    }

    #[test]
    fn evaluate_power_number_number_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Power::new(
            Number::new_node(8.).with_span(Span::new_between(0, 0)),
            Number::new_node(2.).with_span(Span::new_between(2, 2)),
        )
        .with_span(Span::new_between(0, 2))
        .evaluate(options)
        .unwrap();
        assert_eq!(
            result,
            Number::new_node(64.).with_span(Span::new_between(0, 2))
        );
    }

    #[test]
    fn evaluate_power_number_number0() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Power::new(Number::new_node(8.), Number::new_node(0.))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Number::new_node(1.));
    }

    #[test]
    fn evaluate_power_number_number0_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Power::new(
            Number::new_node(8.).with_span(Span::new_between(0, 0)),
            Number::new_node(0.).with_span(Span::new_between(2, 2)),
        )
        .with_span(Span::new_between(0, 2))
        .evaluate(options)
        .unwrap();
        assert_eq!(
            result,
            Number::new_node(1.).with_span(Span::new_between(0, 2))
        );
    }

    #[test]
    fn evaluate_power_number0_number() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Power::new(Number::new_node(0.), Number::new_node(2.))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Number::new_node(0.));
    }

    #[test]
    fn evaluate_power_number0_number_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Power::new(
            Number::new_node(0.).with_span(Span::new_between(0, 0)),
            Number::new_node(2.).with_span(Span::new_between(2, 2)),
        )
        .with_span(Span::new_between(0, 2))
        .evaluate(options)
        .unwrap();
        assert_eq!(
            result,
            Number::new_node(0.).with_span(Span::new_between(0, 2))
        );
    }

    #[test]
    fn evaluate_power_matrix_number_0() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(0.0).into())
            .evaluate(options)
            .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new(1.0).into(),
                    Number::new(0.0).into(),
                    Number::new(0.0).into(),
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                    Number::new(0.0).into(),
                    Number::new(0.0).into(),
                    Number::new(0.0).into(),
                    Number::new(1.0).into(),
                ],
            )
            .into()
        );
    }

    #[test]
    fn evaluate_power_matrix_number_1() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(1.0).into())
            .evaluate(options)
            .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                    Number::new(4.0).into(),
                    Number::new(5.0).into(),
                    Number::new(6.0).into(),
                    Number::new(7.0).into(),
                    Number::new(8.0).into(),
                    Number::new(9.0).into(),
                ],
            )
            .into()
        );
    }

    #[test]
    fn evaluate_power_matrix_number_2() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(2.0).into())
            .evaluate(options)
            .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new(30.0).into(),
                    Number::new(36.0).into(),
                    Number::new(42.0).into(),
                    Number::new(66.0).into(),
                    Number::new(81.0).into(),
                    Number::new(96.0).into(),
                    Number::new(102.0).into(),
                    Number::new(126.0).into(),
                    Number::new(150.0).into(),
                ],
            )
            .into()
        );
    }

    #[test]
    fn evaluate_power_matrix_number_3() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(3.0).into())
            .evaluate(options)
            .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new(468.0).into(),
                    Number::new(576.0).into(),
                    Number::new(684.0).into(),
                    Number::new(1062.0).into(),
                    Number::new(1305.0).into(),
                    Number::new(1548.0).into(),
                    Number::new(1656.0).into(),
                    Number::new(2034.0).into(),
                    Number::new(2412.0).into(),
                ],
            )
            .into()
        );
    }

    #[test]
    fn evaluate_power_matrix_number_4() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(4.0).into())
            .evaluate(options)
            .unwrap();

        assert_eq!(
            result,
            Tensor::new_with_shape(
                vec![3, 3],
                vec![
                    Number::new(7560.0).into(),
                    Number::new(9288.0).into(),
                    Number::new(11016.0).into(),
                    Number::new(17118.0).into(),
                    Number::new(21033.0).into(),
                    Number::new(24948.0).into(),
                    Number::new(26676.0).into(),
                    Number::new(32778.0).into(),
                    Number::new(38880.0).into(),
                ],
            )
            .into()
        );
    }

    #[test]
    fn evaluate_power_matrix_number_err_not_an_integer() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(4.1).into())
            .evaluate(options)
            .unwrap_err();

        assert_eq!(result, Error::not_an_integer());
    }

    #[test]
    fn evaluate_power_matrix_number_err_not_positive() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let matrix = Tensor::new_with_shape(
            vec![3, 3],
            vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
                Number::new(4.0).into(),
                Number::new(5.0).into(),
                Number::new(6.0).into(),
                Number::new(7.0).into(),
                Number::new(8.0).into(),
                Number::new(9.0).into(),
            ],
        );

        let result = Power::new(matrix.into(), Number::new(-1.0).into())
            .evaluate(options)
            .unwrap_err();

        assert_eq!(result, Error::not_positive());
    }
}
