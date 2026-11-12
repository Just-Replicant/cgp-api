//! REST error type and raisers. Application policy — not part of `cgp-orm`.

use core::fmt::Display;

use axum::http::StatusCode;
use cgp_fork::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;
use cgp_orm::error::ErrEntityNotFound;

/// An HTTP status and message, convertible into Axum's `(StatusCode, String)`.
#[derive(Debug, Clone)]
pub struct RestError {
    /// HTTP status.
    pub status: StatusCode,
    /// Human-readable message.
    pub message: String,
}

impl From<RestError> for (StatusCode, String) {
    fn from(error: RestError) -> Self {
        (error.status, error.message)
    }
}

/// Map [`ErrEntityNotFound`] to 404.
pub struct RaiseRestNotFound;

#[cgp_impl(RaiseRestNotFound)]
#[use_type(HasErrorType.{Error = RestError})]
impl ErrorRaiser<ErrEntityNotFound> {
    fn raise_error(e: ErrEntityNotFound) -> RestError {
        RestError {
            status: StatusCode::NOT_FOUND,
            message: e.to_string(),
        }
    }
}

/// Map any [`Display`] source to 500.
pub struct RaiseRestInternal;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseRestInternal
where
    Context: HasErrorType<Error = RestError>,
    E: Display,
{
    fn raise_error(e: E) -> RestError {
        RestError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

/// Map any [`Display`] source to 400.
pub struct RaiseRestBadRequest;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseRestBadRequest
where
    Context: HasErrorType<Error = RestError>,
    E: Display,
{
    fn raise_error(e: E) -> RestError {
        RestError {
            status: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        }
    }
}

/// Prefix `{detail}: {message}` and keep the original status.
pub struct WrapRestError;

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for WrapRestError
where
    Context: HasErrorType<Error = RestError>,
    Detail: Display,
{
    fn wrap_error(error: RestError, detail: Detail) -> RestError {
        RestError {
            status: error.status,
            message: format!("{detail}: {}", error.message),
        }
    }
}
