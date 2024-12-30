use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    NONE,
}

impl HttpMethod {
    pub fn get_by_string(input: &str) -> Self {
        match input {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            _ => Self::NONE,
        }
    }
}

#[derive(PartialEq)]
pub enum HttpResponseCode {
    Success = 200,
    NotFound = 404,
    ServerError = 500,
}

impl fmt::Display for HttpResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpResponseCode::Success => write!(f, "200 OK"),
            HttpResponseCode::NotFound => write!(f, "404 Not Found"),
            HttpResponseCode::ServerError => write!(f, "500 Internal Server Error"),
        }?;
        Ok(())
    }
}
