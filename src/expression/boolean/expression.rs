use std::str::FromStr;

use super::factor::Factor;
use crate::expression;

// Represents a term, which can consists of one, or multiple factors which
// can be divided or multiplied.
expression!(Expression<bool, Factor> | ExpressionOperator: And => &&, Or => ||);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::expression::common::{Expression as ExpressionTrait, Reducible};

    #[test]
    fn test_expression_reducibility() {
        let expression_str = "UNKNOWN || false";
        let expression = Expression::parse(expression_str).unwrap().1;
        let mut variables_table = HashMap::new();
        variables_table.insert("UNKNOWN".into(), true);
        let actual = expression.reduce(&variables_table).unwrap();

        assert_eq!(true, actual);
    }

    #[test]
    fn test_expression_reducibility_complex() {
        let expression_str = "!UNKNOWN || UNKNOWN && true || false && VALUE";
        let expression = Expression::parse(expression_str).unwrap().1;

        let mut variables = HashMap::new();
        variables.insert("UNKNOWN".into(), true);
        variables.insert("VALUE".into(), false);

        let actual = expression.reduce(&variables).unwrap();

        assert_eq!(false, actual);
    }
}
