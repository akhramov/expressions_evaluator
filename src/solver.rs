use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::expression::boolean::parse as parse_boolean_expression;
use crate::expression::boolean::Expression as BoolExpression;

use crate::expression::arithmetic::parse as parse_arithmetic_expression;
use crate::expression::arithmetic::Expression as ArithmeticExpression;

use crate::expression::Reducible;

const BASE_BOOLEAN: [&str; 3] = [
    "A && B && !C => H = M",
    "A && B && C  => H = P",
    "!A && B && C => H = T",
];

const BASE_ARITHMETIC: [&str; 3] = [
    "H = M => K = D + (D * E / 10)",
    "H = P => K = D + (D * (E - F) / 25.5)",
    "H = T => K = D - (D * F / 30)",
];

/// The backbone of the application. Solves boolean & arithmetic
/// expressions.
pub struct Solver {
    boolean: Vec<(BoolExpression, String)>,
    arithmetic: Vec<(ArithmeticExpression, String)>,
}

impl Solver {
    /// Try to parse the given expression as either boolean or arithmetic.
    pub fn add(&mut self, input: &str) -> Result<()> {
        let Self {
            boolean,
            arithmetic,
        } = self;

        if let Ok((_, result)) = parse_boolean_expression(input) {
            return Ok(boolean.push(result));
        }

        if let Ok((_, result)) = parse_arithmetic_expression(input) {
            return Ok(arithmetic.push(result));
        }

        anyhow::bail!("Unable to parse the expression {}", input)
    }

    pub fn add_all(&mut self, expressions: &Vec<String>) -> Result<()> {
        for expression in expressions {
            self.add(&expression)?;
        }

        Ok(())
    }

    /// Given variable tables, solve stored expressions.
    pub fn solve(
        &self,
        bool_vars: HashMap<String, bool>,
        float_vars: HashMap<String, f64>,
    ) -> Result<(String, f64)> {
        // First, find a truthy boolean expression
        let (_, label) = self
            .boolean
            .iter()
            .rev()
            .try_find(|(expression, _)| expression.reduce(&bool_vars))?
            .context("Unable to find the solution")?;

        // Then, find a matching float expression
        let (expression, _) = self
            .arithmetic
            .iter()
            .rev()
            .find(|(_, float_label)| label == float_label)
            .context("Unable to find the solution")?;

        Ok((label.into(), expression.reduce(&float_vars)?))
    }
}

impl Default for Solver {
    fn default() -> Self {
        let boolean = BASE_BOOLEAN.iter().fold(vec![], |mut acc, string| {
            // Unwrap is justified, because our base expressions are okay.
            let result = parse_boolean_expression(string).unwrap().1;
            acc.push(result);

            acc
        });

        let arithmetic =
            BASE_ARITHMETIC.iter().fold(vec![], |mut acc, string| {
                // Unwrap is justified, because our base expressions are okay.
                let result = parse_arithmetic_expression(string).unwrap().1;
                acc.push(result);

                acc
            });

        Self {
            boolean,
            arithmetic,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn variable_tables() -> (HashMap<String, bool>, HashMap<String, f64>) {
        let mut bool_vars = HashMap::new();
        bool_vars.insert("A".into(), false);
        bool_vars.insert("B".into(), true);
        bool_vars.insert("C".into(), true);

        let mut arithmetic_vars = HashMap::new();
        arithmetic_vars.insert("D".into(), 1.5);
        arithmetic_vars.insert("E".into(), 20.0);
        arithmetic_vars.insert("F".into(), 10.0);

        (bool_vars, arithmetic_vars)
    }

    #[test]
    fn test_default_solver() {
        let (bool_vars, arithmetic_vars) = variable_tables();
        let solver = Solver::default();
        let solution = solver.solve(bool_vars, arithmetic_vars).unwrap();

        assert_eq!(("T".into(), 1.0), solution);
    }

    // Newly added values take precedence over the basic ones.
    #[test]
    fn test_adding_expressions() {
        let (bool_vars, arithmetic_vars) = variable_tables();
        let mut solver = Solver::default();

        solver
            .add("H = M => K = (E * D * D / (E * (D * (F + E))))")
            .unwrap();
        solver.add("A || !A => H = M").unwrap();
        let solution = solver.solve(bool_vars, arithmetic_vars).unwrap();

        assert_eq!(("M".into(), 0.05), solution);
    }

    #[test]
    fn test_unsolvable() {
        let (mut bool_vars, arithmetic_vars) = variable_tables();
        let solver = Solver::default();

        bool_vars.insert("B".into(), false);

        let solution = solver.solve(bool_vars, arithmetic_vars).unwrap_err();

        assert_eq!("Unable to find the solution", format!("{}", solution));
    }
}
