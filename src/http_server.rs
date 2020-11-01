mod request;
mod response;
mod error;

use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::solver::Solver;
use self::error::Error;

use request::Request;
use response::Response;


#[post("/solution", data = "<request>")]
fn solution(request: Json<Request>) -> Result<Json<Response>, Error> {
    let mut solver = Solver::default();
    let (bool_vars, float_vars) = (&request.variables).into();
    solver.add_all(&request.additional_rules)?;

    let solution = solver.solve(bool_vars, float_vars)?;

    Ok(Json(solution.into()))
}

pub fn server() -> Rocket {
    rocket::ignite().mount("/", routes![solution])
}

#[cfg(test)]
mod test {
    use super::server;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    // A convinience macro to make a request with the specified body fixture.
    macro_rules! make_request {
        ($var:ident, $str:expr) => {
            let client = Client::new(server()).expect("valid rocket instance");
            let mut $var = client
                .post("/solution")
                .header(ContentType::JSON)
                .body(include_str!(concat!("./http_server/test_fixtures/", $str, ".json")))
                .dispatch();
        }
    }

    #[test]
    fn test_happy_path() {
        make_request!(response, "happy_path");

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"H":"P","K":1.05}"#.into()));
    }

    #[test]
    fn test_unknown_variable_supplied() {
        make_request!(response, "unknown_variable");
        let error_text = r#"{"reason":"Variable UNKNOWN is undefined"}"#;

        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert_eq!(response.body_string(), Some(error_text.into()));
    }

    #[test]
    fn test_parsing_error() {
        make_request!(response, "parsing_error");
        let error_text =
            r#"{"reason":"Unable to parse the expression A && 13 => H = P"}"#;

        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert_eq!(response.body_string(), Some(error_text.into()));
    }

    #[test]
    fn test_no_match() {
        make_request!(response, "no_match");
        let error_text =
            r#"{"reason":"Unable to find the solution"}"#;

        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert_eq!(response.body_string(), Some(error_text.into()));
    }

    #[test]
    fn test_malformed_request() {
        make_request!(response, "malformed");

        assert_eq!(response.status(), Status::BadRequest);
    }
}
