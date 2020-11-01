use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

pub trait Expression: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;

    fn parse_with_operator<T: Expression, S: Operator>(
        input: &str,
    ) -> IResult<&str, (T, Vec<(S, T)>)> {
        pair(T::parse, many0(variable_with_operator))(input)
    }
}

#[derive(PartialEq, Debug)]
pub struct ExpressionWithOperator<T: Expression, O: Operator> {
    pub head: T,
    pub tail: Vec<(O, T)>,
}

impl<T, O> Expression for ExpressionWithOperator<T, O>
where
    T: Expression,
    O: Operator,
{
    fn parse(input: &str) -> IResult<&str, Self> {
        map(Self::parse_with_operator::<T, O>, |(head, tail)| Self {
            head,
            tail,
        })(input)
    }
}

pub trait Operator: FromStr {
    fn parser() -> Box<dyn Fn(&str) -> IResult<&str, &str>>;

    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(Self::parser(), Self::from_str)(input)
    }
}

pub trait Reducible<N> {
    fn reduce(&self, variables_table: &HashMap<String, N>) -> Result<N>;
}

fn variable_with_operator<T, S>(input: &str) -> IResult<&str, (S, T)>
where
    T: Expression,
    S: Operator,
{
    pair(S::parse, T::parse)(input)
}

/// The basic idea of this macro is to generate the Expression type given a
/// set of operations.
///
/// The expression can then be parsed or evaluated (aka reduced) using
/// `Expression` & `Reducible` traits correspondingly.
#[macro_export]
macro_rules! expression {
    ($expression_type:ident<$type:ty, $consists_of:ty> | $name:ident: $($variant:ident => $op:tt),*) => {
        pub type $expression_type =
            crate::expression::common::ExpressionWithOperator<$consists_of, $name>;

        #[derive(PartialEq, Debug)]
        pub enum $name {
            $( $variant, )*
        }

        impl FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input {
                    $( stringify!($op) => Ok($name::$variant), )*
                    _ => anyhow::bail!("Operator {} is not recognized", input)
                }
            }
        }

        impl crate::expression::common::Operator for $name {
            fn parser() -> Box<dyn Fn(&str) -> nom::IResult<&str, &str>> {
                use nom::bytes::complete::tag;
                use nom::branch::alt;

                Box::new(|input: &str| {
                    alt(($( tag(stringify!($op)), )*))(input)
                })
            }
        }

        impl crate::expression::common::Reducible<$type> for $expression_type {
            fn reduce(&self, variables_table: &std::collections::HashMap<String, $type>)
                      -> anyhow::Result<$type> {
                let init = self.head.reduce(variables_table)?;

                self.tail.iter().try_fold(init, |acc, (operator, item)| {
                    match operator {
                        $( $name::$variant => {
                            item.reduce(variables_table)
                                .map(|result| acc $op result)
                        }, )*
                    }
                })
            }
        }

    }
}
