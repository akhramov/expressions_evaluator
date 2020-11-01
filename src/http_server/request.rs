use std::collections::HashMap;

use serde::Deserialize;

/// Represents HTTP body request JSON.
#[derive(Deserialize)]
pub struct Request {
    pub additional_rules: Vec<String>,
    pub variables: Variables,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Variables {
    a: bool,
    b: bool,
    c: bool,
    d: f64,
    e: i64,
    f: i64,
}

// Solver is generic, so we need an ad-hoc converter to adhere to
// solver's params.
impl From<&Variables> for (HashMap<String, bool>, HashMap<String, f64>) {
    fn from(variables: &Variables) -> Self {
        let mut bool_vars = HashMap::new();
        bool_vars.insert("A".into(), variables.a);
        bool_vars.insert("B".into(), variables.b);
        bool_vars.insert("C".into(), variables.c);

        let mut arithmetic_vars = HashMap::new();
        arithmetic_vars.insert("D".into(), variables.d);
        arithmetic_vars.insert("E".into(), variables.e as _);
        arithmetic_vars.insert("F".into(), variables.f as _);

        (bool_vars, arithmetic_vars)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_request_deserialization() {
        let data = r#"
            {
              "variables": {
                "A": true,
                "B": true,
                "C": false,
                "D": 1.05,
                "E": 4,
                "F": 42
              },
              "additional_rules": [
                "A && B => H = P"
              ]
            }
        "#;

        let request: Request = serde_json::from_str(data).unwrap();

        assert_eq!(request.additional_rules, vec!["A && B => H = P"]);
        assert_eq!(request.variables.c, false);
    }
}
