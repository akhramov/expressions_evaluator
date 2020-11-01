/// Expressions consist of terms, which are added and subtracted.
/// Terms consist of factors, which are multiplied and divided.
use std::str::FromStr;

use super::factor::Factor;
use crate::expression;

// Represents a term, which can consists of one, or multiple factors which
// can be divided or multiplied.
expression!(Term<f64, Factor> | TermOperator: Multiply => *, Divide => /);

// Represents an expression, which can consists of one, or multiple terms
// which can be added or subtracted.
expression!(Expression<f64, Term> | ExpressionOperator: Add => +, Subtract => -);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ExpressionOperator::*, TermOperator::*, *};
    use crate::expression::arithmetic::factor::Factor::*;
    use crate::expression::common::{
        Expression as ExpressionTrait, ExpressionWithOperator, Reducible,
    };

    #[test]
    fn test_expression_reducibility() {
        let expression_str = "42 + 4 * (5 / 2 - 1) - 10 * 0.625 + 1 / 2";
        let expression = Expression::parse(expression_str).unwrap().1;
        let actual = expression.reduce(&HashMap::new()).unwrap();

        assert_eq!(42.25, actual);
    }

    #[test]
    fn test_expression_reducibility_variables() {
        let expression_str = "42 + 4 * (5 / UNKNOWN - 1) - 10 * VALUE + 1 / 2";
        let expression = Expression::parse(expression_str).unwrap().1;
        let mut variables = HashMap::new();
        variables.insert("UNKNOWN".into(), 0.8);
        variables.insert("VALUE".into(), -0.4);

        let actual = expression.reduce(&variables).unwrap();

        assert_eq!(67.5, actual);
    }

    #[test]
    fn test_expression_undefined_vars() {
        let expression_str = "-5 + UNKNOWN";
        let expression = Expression::parse(expression_str).unwrap().1;
        let variables = HashMap::new();

        let actual = expression.reduce(&variables).unwrap_err();

        assert_eq!("Variable UNKNOWN is undefined", format!("{}", actual));
    }

    #[test]
    fn test_expression_divide_by_zero() {
        let expression_str = "-5 / 0";
        let expression = Expression::parse(expression_str).unwrap().1;
        let variables = HashMap::new();

        let actual = expression.reduce(&variables).unwrap();

        // Yeah, perfectly fine. Thanks IEEE 754.
        assert_eq!(f64::NEG_INFINITY, actual);
    }

    #[test]
    fn test_expression_parsing() {
        let expected = ExpressionWithOperator {
            head: ExpressionWithOperator {
                head: Constant(42.0),
                tail: vec![],
            },
            tail: vec![(
                Add,
                ExpressionWithOperator {
                    head: Constant(4.0),
                    tail: vec![(
                        Multiply,
                        ExpressionInParens(Box::new(ExpressionWithOperator {
                            head: ExpressionWithOperator {
                                head: Constant(5.0),
                                tail: vec![(Divide, Variable("Foo".into()))],
                            },
                            tail: vec![(
                                Subtract,
                                ExpressionWithOperator {
                                    head: Variable("Bar".into()),
                                    tail: vec![],
                                },
                            )],
                        })),
                    )],
                },
            )],
        };

        let actual = Expression::parse("42 + 4 * (5 / Foo - Bar)").unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_term_parsing_multiplication() {
        let expected = Term {
            tail: vec![(Multiply, Factor::Constant(4.0))],
            head: Factor::Constant(42.0),
        };

        assert_eq!(Ok(("", expected)), Term::parse("42 * 4"));
    }

    #[test]
    fn test_term_parsing_constant() {
        let expected = Term {
            tail: vec![],
            head: Factor::Constant(42.0),
        };

        assert_eq!(Ok(("", expected)), Term::parse("42"));
    }

    #[test]
    fn test_term_parsing_mixed() {
        let expected = Term {
            tail: vec![
                (Multiply, Factor::Variable("Foobar".into())),
                (Divide, Factor::Constant(55.0)),
            ],
            head: Factor::Constant(42.0),
        };

        assert_eq!(Ok(("", expected)), Term::parse("42 * Foobar / 55"));
    }

    #[test]
    fn test_term_parsing_invalid() {
        let tail = vec![(Multiply, Factor::Variable("Foobar".into()))];

        let expected = Term {
            tail,
            head: Factor::Constant(42.0),
        };

        let remainder = "+ $#&#7#R!";
        let expression = format!("42 * Foobar {}", remainder);

        assert_eq!(Ok((remainder, expected)), Term::parse(&expression));
    }
}
