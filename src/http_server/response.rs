use serde::Serialize;

// Represenets HTTP body response JSON
#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Response {
    h: String,
    k: f64,
}

// Convert solver's result to response
impl From<(String, f64)> for Response {
    fn from((h, k): (String, f64)) -> Self {
        Self { h, k }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_response_serialization() {
        let expected = r#"{"H":"M","K":0.133333}"#;

        let response: Response = ("M".into(), 0.133333).into();
        let actual = serde_json::to_string(&response).unwrap();

        assert_eq!(expected, actual);
    }
}
