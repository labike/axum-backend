#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExists,
    UserNotFound,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthorized,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password exceeds maximum length of {}", max_length)
            }
            ErrorMessage::HashingError => "Error occurred while hashing the password".to_string(),
            ErrorMessage::InvalidToken => "Invalid token provided".to_string(),
            ErrorMessage::ServerError => "Internal server error".to_string(),
            ErrorMessage::WrongCredentials => "Incorrect username or password".to_string(),
            ErrorMessage::EmailExists => "Email already exists".to_string(),
            ErrorMessage::UserNotFound => "User not found".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::PermissionDenied => "Permission denied".to_string(),
            ErrorMessage::UserNotAuthorized => "User not authorized".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid hash format".to_string(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StateCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StateCode) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StateCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StateCode::BAD_REQUEST,
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StateCode::CONFLICT,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StateCode::UNAUTHORIZED,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });
        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}

impl std::error::Error for HttpError {
    
}