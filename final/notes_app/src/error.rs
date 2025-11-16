use rocket::http::Status;

/// Convenience alias for route return types.
pub type RouteResult<T> = Result<T, Status>;

/// Map any database-related error to a generic 500 status.
pub fn db_error<E>(_err: E) -> Status {
    Status::InternalServerError
}

/// Map any pool/connection-related error to a generic 503 status.
pub fn pool_error<E>(_err: E) -> Status {
    Status::ServiceUnavailable
}

