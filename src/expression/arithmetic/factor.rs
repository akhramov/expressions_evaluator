/// This module is used to parse variables in arithmetic expressions,
/// including constants and nested parenthezised expressions.
use std::collections::HashMap;

use anyhow::{Context, Result};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space0};
use nom::combinator::map;
use nom::number::complete as number;
use nom::sequence::delimited;
use nom::IResult;

use super::expression::Expression as ArithmeticExpression;
use crate::expression::common::{Expression, Reducible};

/// An enumeration representing either a variable or a constant number.
#[derive(PartialEq, Debug)]
pub enum Factor {
    Variable(String),
    Constant(f64),
    ExpressionInParens(Box<ArithmeticExpression>),
}

impl Reducible<f64> for Factor {
    fn reduce(&self, variables_table: &HashMap<String, f64>) -> Result<f64> {
        match self {
            Self::Constant(constant) => Ok(*constant),
            Self::Variable(string) => variables_table
                .get(string)
                .map(|&val| val)
                .with_context(|| format!("Variable {} is undefined", string)),
            Self::ExpressionInParens(expression) => {
                expression.reduce(variables_table)
            },
        }
    }
}

impl Expression for Factor {
    /// Try convert the input into a number or variable name, or a
    /// parenthesized expression.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::expression::arithmetic::factor::Factor;
    ///
    /// let variable = Factor::parse(" 42.0000 ").unwrap().1;
    /// assert_eq!(variable, Factor::Constant(42.0));
    /// ```
    fn parse(input: &str) -> IResult<&str, Self> {
        delimited(space0, alt((expr, constant, variable)), space0)(input)
    }
}

fn expr(input: &str) -> IResult<&str, Factor> {
    let expr = delimited(tag("("), Expression::parse, tag(")"));

    map(expr, |expression| {
        Factor::ExpressionInParens(Box::new(expression))
    })(input)
}

fn constant(input: &str) -> IResult<&str, Factor> {
    map(number::double, |constant| Factor::Constant(constant))(input)
}

fn variable(input: &str) -> IResult<&str, Factor> {
    map(alpha1, |name: &str| Factor::Variable(name.into()))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expression::arithmetic::{
        expression::{Expression, Term, TermOperator::*},
        factor::Factor::*,
    };
    use crate::expression::common::Expression as ExpressionTrait;

    #[test]
    fn test_parsing_constant() {
        assert_eq!(Ok(("", Factor::Constant(42.0))), Factor::parse("42"));
    }

    #[test]
    fn test_parsing_variable() {
        assert_eq!(
            Ok(("", Factor::Variable("VARNAME".into()))),
            Factor::parse("VARNAME")
        );
    }

    #[test]
    fn test_invalid_input() {
        use nom::error::ErrorKind;
        use nom::Err::Error;

        assert_eq!(
            Err(Error(("/$#%*", ErrorKind::Alpha))),
            Factor::parse("/$#%*")
        );
    }

    #[test]
    fn test_parsing_parenthized_expression() {
        let expected = ExpressionInParens(Box::new(Expression {
            head: Term {
                head: Factor::Constant(5.0),
                tail: vec![(Multiply, Factor::Variable("VARNAME".into()))],
            },
            tail: vec![],
        }));

        assert_eq!(expected, Factor::parse("(5 * VARNAME)").unwrap().1,);
    }
}
