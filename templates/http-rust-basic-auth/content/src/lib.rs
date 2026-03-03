use base64::{engine::general_purpose, Engine as _};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A Spin HTTP component with HTTP Basic Authentication.
///
/// Credentials are read from Spin variables at runtime:
///   - `username` (default: "admin")
///   - `password` (required, no default)
///
/// Set them via environment variables, a .env file, or at deploy time:
///   SPIN_VARIABLE_USERNAME=alice SPIN_VARIABLE_PASSWORD=secret spin up
#[http_component]
fn handle_{{project-name | snake_case}}(req: Request) -> anyhow::Result<impl IntoResponse> {
    let username = spin_sdk::variables::get("username")?;
    let password = spin_sdk::variables::get("password")?;

    match validate_basic_auth(&req, &username, &password) {
        Ok(user) => {
            println!("Authenticated request from '{user}'");
            Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body(format!("Hello, {user}!"))
                .build())
        }
        Err(reason) => {
            println!("Unauthorized: {reason}");
            Ok(Response::builder()
                .status(401)
                .header("www-authenticate", "Basic realm=\"Spin App\"")
                .header("content-type", "text/plain")
                .body(format!("Unauthorized: {reason}"))
                .build())
        }
    }
}

/// Extracts and validates `Authorization: Basic <base64(username:password)>`.
fn validate_basic_auth(req: &Request, valid_username: &str, valid_password: &str) -> Result<String, String> {
    let auth = req
        .header("authorization")
        .and_then(|h| h.as_str())
        .ok_or("Missing Authorization header")?;

    let encoded = auth
        .strip_prefix("Basic ")
        .ok_or("Authorization header must start with 'Basic '")?;

    let decoded_bytes = general_purpose::STANDARD
        .decode(encoded.trim())
        .map_err(|_| "Invalid base64 encoding")?;

    let decoded =
        std::str::from_utf8(&decoded_bytes).map_err(|_| "Credentials are not valid UTF-8")?;

    let (username, password) = decoded
        .split_once(':')
        .ok_or("Credentials must be formatted as 'username:password'")?;

    if username == valid_username && password == valid_password {
        Ok(username.to_string())
    } else {
        Err("Invalid username or password".to_string())
    }
}
