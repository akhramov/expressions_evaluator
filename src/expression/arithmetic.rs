mod expression;
mod factor;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space0};
use nom::combinator::map;
use nom::sequence::{delimited, pair};
use nom::IResult;

pub use self::expression::Expression;
use crate::expression::common::Expression as ExpressionTrait;

/// Parses the whole arithmetic expression, such as `H = M => K = D * 2`.
pub fn parse(input: &str) -> IResult<&str, (Expression, String)> {
    let parser = pair(parse_matcher_clause, Expression::parse);

    map(parser, |(string, expression)| (expression, string))(input)
}

/// Parses a matcher clause, such as `H = M => K =` and returns the
/// value of H corresponding to the clause (M in the case of the example).
fn parse_matcher_clause(input: &str) -> IResult<&str, String> {
    delimited(tag("H = "), variable, tag("=> K = "))(input)
}

fn variable(input: &str) -> IResult<&str, String> {
    map(delimited(space0, alpha1, space0), |string: &str| {
        string.into()
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matcher_clause_parsing() {
        let actual =
            parse_matcher_clause("H = M => K = D * 2 / A * B - 5").unwrap();

        assert_eq!(("D * 2 / A * B - 5", "M".into()), actual);
    }

    #[test]
    fn test_integration_parser() {
        use crate::expression::common::Reducible;
        use std::collections::HashMap;

        let input = "H = M => K = 21 * 4 / 2 - 10";

        let parsed = parse(input).unwrap().1;
        let result = parsed.0.reduce(&HashMap::new()).unwrap();

        assert_eq!(result, 32.0);
        assert_eq!(parsed.1, "M");
    }
}
