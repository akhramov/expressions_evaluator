mod expression;
mod factor;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space0};
use nom::combinator::map;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

pub use self::expression::Expression;
use crate::expression::common::Expression as ExpressionTrait;

/// Parses the whole boolean expression, such as `A && B && !C => H = M`.
pub fn parse(input: &str) -> IResult<&str, (Expression, String)> {
    pair(Expression::parse, parse_matcher_clause)(input)
}

/// Parses a matcher clause, such as `=> H = M` and returns the
/// value of H corresponding to the clause (M in the case of the example).
fn parse_matcher_clause(input: &str) -> IResult<&str, String> {
    preceded(tag("=> H ="), variable)(input)
}

fn variable(input: &str) -> IResult<&str, String> {
    map(delimited(space0, alpha1, space0), String::from)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matcher_clause_parsing() {
        let actual = parse_matcher_clause("=> H = M").unwrap();

        assert_eq!(("", "M".into()), actual);
    }

    #[test]
    fn test_matchers_integration() {
        let input = "A && B && !C => H = M";
        let result = parse(input).unwrap().1;

        assert_eq!(result.1, "M");
    }
}
