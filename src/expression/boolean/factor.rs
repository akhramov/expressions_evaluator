/// This module is used to parse variables in boolean expressions,
/// including constants (true / false).
use std::collections::HashMap;

use anyhow::{Context, Result};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space0};
use nom::combinator::{map, opt};
use nom::sequence::{delimited, pair};
use nom::IResult;

use crate::expression::common::{Expression, Reducible};

/// An enumeration representing either a variable or a constant number.
#[derive(PartialEq, Debug)]
pub enum Factor {
    Variable(String),
    NegatedVariable(String),
    Constant(bool),
}

impl Reducible<bool> for Factor {
    fn reduce(&self, variables_table: &HashMap<String, bool>) -> Result<bool> {
        fn fetch_variable(
            variable: &String,
            table: &HashMap<String, bool>,
            negated: bool,
        ) -> Result<bool> {
            table
                .get(variable)
                .map(|&val| if negated { !val } else { val })
                .with_context(|| format!("Variable {} is undefined", variable))
        }

        match self {
            Self::Constant(constant) => Ok(*constant),
            Self::Variable(string) => {
                fetch_variable(string, variables_table, false)
            },
            Self::NegatedVariable(string) => {
                fetch_variable(string, variables_table, true)
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
    /// use crate::expression::boolean::factor::Factor;
    ///
    /// let variable = Factor::parse(" !true ").unwrap().1;
    /// assert_eq!(variable, Factor::Constant(false));
    /// ```
    fn parse(input: &str) -> IResult<&str, Self> {
        delimited(space0, alt((constant, variable)), space0)(input)
    }
}

fn constant(input: &str) -> IResult<&str, Factor> {
    map(
        pair(opt(tag("!")), alt((tag("true"), tag("false")))),
        |parsed| {
            let constant = match parsed {
                (Some(_), "true") | (None, "false") => false,
                (Some(_), "false") | (None, "true") => true,
                _ => unreachable!(), // Really is. It's either true or false.
            };

            Factor::Constant(constant)
        },
    )(input)
}

fn variable(input: &str) -> IResult<&str, Factor> {
    map(
        pair(opt(tag("!")), alpha1),
        |parsed: (Option<&str>, &str)| match parsed {
            (Some(_), negated) => Factor::NegatedVariable(negated.into()),
            (None, variable) => Factor::Variable(variable.into()),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expression::common::Expression as ExpressionTrait;

    #[test]
    fn test_parsing_constant() {
        assert_eq!(Ok(("", Factor::Constant(false))), Factor::parse("!true"));
        assert_eq!(Ok(("", Factor::Constant(false))), Factor::parse("false"));
        assert_eq!(Ok(("", Factor::Constant(true))), Factor::parse("true"));
    }

    #[test]
    fn test_parsing_variable() {
        assert_eq!(
            Ok(("", Factor::Variable("VARNAME".into()))),
            Factor::parse("VARNAME")
        );

        assert_eq!(
            Ok(("", Factor::NegatedVariable("VARNAME".into()))),
            Factor::parse("!VARNAME")
        );
    }
}
