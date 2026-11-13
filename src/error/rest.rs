//! REST error type and raisers. Application policy — not part of `cgp-orm`.
//!
//! [`RaiseRestInternal`] is registered for [`sea_orm::DbErr`], `String`, and
//! `&str`. Its client message is a fixed `"internal error"` so SQL and other
//! unexpected text do not leak into the HTTP body. The original is available
//! in-process through [`RestError`]'s [`Error::source`](core::error::Error::source).
//!
//! This crate does not add a logger or a debug-errors feature: there is no
//! `tracing` dependency, and an off-by-default leak switch gets left on.
//! The raiser keys are already `open`. An application that wants the raw
//! text on a private network overrides that one key
//! (`@ErrorRaiserComponent.::sea_orm::DbErr`, and the `String` / `&str`
//! keys if needed) with a raiser of its own.

use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use std::sync::Arc;

use axum::http::StatusCode;
use cgp_fork::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;
use cgp_orm::error::ErrEntityNotFound;

use crate::error::{INTERNAL_ERROR_MESSAGE, UnexpectedMessage};

/// An HTTP status, a client-facing message, and an optional original source.
///
/// [`Display`] and [`From<RestError>`] for `(StatusCode, String)` use only
/// [`Self::message`] — the body a client is allowed to see. The original
/// error, when present, is available through [`Error::source`].
///
/// [`PartialEq`] and [`Eq`] compare [`Self::status`] and [`Self::message`]
/// only; the source is ignored.
pub struct RestError {
    /// HTTP status.
    pub status: StatusCode,
    /// Client-facing message. Never a raw [`sea_orm::DbErr`] when produced
    /// by [`RaiseRestInternal`].
    pub message: String,
    source: Option<Arc<dyn Error + Send + Sync + 'static>>,
}

impl RestError {
    /// Build a REST error with a client-facing `message` and no source.
    #[must_use]
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            source: None,
        }
    }

    /// Attach an original error, available through [`Error::source`].
    #[must_use]
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self
    }

    /// HTTP 500 with a fixed [`INTERNAL_ERROR_MESSAGE`] client message.
    ///
    /// The original is kept as [`Error::source`].
    #[must_use]
    pub fn internal(source: impl Error + Send + Sync + 'static) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR_MESSAGE).with_source(source)
    }

    /// The original error, if one was attached.
    #[must_use]
    pub fn original(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        self.source.as_deref()
    }
}

impl Debug for RestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RestError")
            .field("status", &self.status)
            .field("message", &self.message)
            .field(
                "source",
                &self.source.as_ref().map(|error| error.to_string()),
            )
            .finish()
    }
}

impl Clone for RestError {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            message: self.message.clone(),
            source: self.source.clone(),
        }
    }
}

impl PartialEq for RestError {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.message == other.message
    }
}

impl Eq for RestError {}

impl Display for RestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for RestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|error| error.as_ref() as &(dyn Error + 'static))
    }
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
        RestError::new(StatusCode::NOT_FOUND, e.to_string()).with_source(e)
    }
}

/// Map [`sea_orm::DbErr`], `String`, and `&str` to HTTP 500 with a fixed
/// [`INTERNAL_ERROR_MESSAGE`].
///
/// The original is kept as [`RestError`]'s source. Typed 500s that are
/// safe to show (for example [`cgp_orm::error::ErrUniqueConflict`]) use
/// [`RaiseRestInternalDisplay`] instead.
///
/// To send the raw text to a client (for example on a private network),
/// override this raiser key on a child table — the keys are `open`.
pub struct RaiseRestInternal;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, sea_orm::DbErr> for RaiseRestInternal
where
    Context: HasErrorType<Error = RestError>,
{
    fn raise_error(e: sea_orm::DbErr) -> RestError {
        RestError::internal(e)
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, String> for RaiseRestInternal
where
    Context: HasErrorType<Error = RestError>,
{
    fn raise_error(e: String) -> RestError {
        RestError::internal(UnexpectedMessage(e))
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, &str> for RaiseRestInternal
where
    Context: HasErrorType<Error = RestError>,
{
    fn raise_error(e: &str) -> RestError {
        RestError::internal(UnexpectedMessage(e.to_owned()))
    }
}

/// Map a [`Display`] + [`Error`] source to HTTP 500, using its Display as
/// the client message.
///
/// Used for typed library failures that do not contain SQL
/// ([`cgp_orm::error::ErrUniqueConflict`], missing primary key, and the
/// other mapped 500s). For [`sea_orm::DbErr`] and unexpected strings, use
/// [`RaiseRestInternal`].
pub struct RaiseRestInternalDisplay;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseRestInternalDisplay
where
    Context: HasErrorType<Error = RestError>,
    E: Display + Error + Send + Sync + 'static,
{
    fn raise_error(e: E) -> RestError {
        RestError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).with_source(e)
    }
}

/// Map any [`Display`] source to 400.
pub struct RaiseRestBadRequest;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseRestBadRequest
where
    Context: HasErrorType<Error = RestError>,
    E: Display + Error + Send + Sync + 'static,
{
    fn raise_error(e: E) -> RestError {
        RestError::new(StatusCode::BAD_REQUEST, e.to_string()).with_source(e)
    }
}

/// Prefix `{detail}: {message}` on the client message and keep the original
/// status and source chain.
///
/// The detail is applied to the client message only, so wrapping an
/// internal error cannot leak a [`sea_orm::DbErr`].
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
            source: Some(Arc::new(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::error::Error;

    use axum::http::StatusCode;
    use cgp_fork::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp_fork::prelude::*;
    use cgp_orm::error::{ErrEntityNotFound, ErrUniqueConflict};

    use super::RestError;
    use crate::error::{INTERNAL_ERROR_MESSAGE, UnexpectedMessage};
    use crate::presets::RestErrors;

    struct ErrorApp;

    delegate_components! {
        #[unchecked]
        ErrorApp {
            ErrorTypeProviderComponent:
                <RestErrors as DelegateComponent<ErrorTypeProviderComponent>>::Delegate,
            ErrorRaiserComponent:
                <RestErrors as DelegateComponent<ErrorRaiserComponent>>::Delegate,
            ErrorWrapperComponent:
                <RestErrors as DelegateComponent<ErrorWrapperComponent>>::Delegate,
        }
    }

    #[test]
    fn db_err_is_opaque_to_the_client() {
        let sql = "SELECT * FROM secrets; syntax error at or near \"SELECT\"";
        let db = sea_orm::DbErr::Custom(sql.into());
        let error = ErrorApp::raise_error(db.clone());
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.message, INTERNAL_ERROR_MESSAGE);
        assert_eq!(error.to_string(), INTERNAL_ERROR_MESSAGE);

        let (status, message): (StatusCode, String) = error.clone().into();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(message, INTERNAL_ERROR_MESSAGE);
        assert!(
            !message.contains("SELECT"),
            "client body must not contain SQL: {message}"
        );

        let source = error.source().expect("original DbErr");
        assert!(
            source.to_string().contains("SELECT"),
            "in-process source should keep the SQL, got {}",
            source
        );
        assert!(source.downcast_ref::<sea_orm::DbErr>().is_some());
    }

    #[test]
    fn unexpected_strings_are_opaque_to_the_client() {
        let from_str = ErrorApp::raise_error("sql leaked");
        assert_eq!(from_str.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(from_str.to_string(), INTERNAL_ERROR_MESSAGE);
        let source = from_str.source().expect("original message");
        assert_eq!(source.to_string(), "sql leaked");
        assert_eq!(
            source.downcast_ref::<UnexpectedMessage>(),
            Some(&UnexpectedMessage("sql leaked".into()))
        );

        let from_string = ErrorApp::raise_error(String::from("sql leaked"));
        assert_eq!(from_string.to_string(), INTERNAL_ERROR_MESSAGE);
        assert_eq!(
            from_string.source().map(ToString::to_string).as_deref(),
            Some("sql leaked")
        );

        let pair: (StatusCode, String) = from_string.into();
        assert_eq!(pair.1, INTERNAL_ERROR_MESSAGE);
    }

    #[test]
    fn unique_conflict_uses_its_display_as_the_client_message() {
        let error = ErrorApp::raise_error(ErrUniqueConflict);
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.message, ErrUniqueConflict.to_string());
        assert!(error.source().is_some());
    }

    #[test]
    fn not_found_is_404_with_source() {
        let not_found = ErrEntityNotFound::of::<u32>();
        let entity_name = not_found.entity_name;
        let error = ErrorApp::raise_error(not_found);
        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert_eq!(error.message, format!("entity {entity_name} not found"));
        assert!(
            error
                .source()
                .and_then(|source| source.downcast_ref::<ErrEntityNotFound>())
                .is_some()
        );
    }

    #[test]
    fn wrap_does_not_leak_db_err_into_the_client_message() {
        let db = sea_orm::DbErr::Custom("SELECT leaked".into());
        let error = ErrorApp::raise_error(db);
        let wrapped = ErrorApp::wrap_error(error, "load user");
        assert_eq!(wrapped.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            wrapped.to_string(),
            format!("load user: {INTERNAL_ERROR_MESSAGE}")
        );
        assert!(
            !wrapped.message.contains("SELECT"),
            "wrapped client body leaked SQL: {}",
            wrapped.message
        );
        let source = wrapped.source().expect("previous RestError");
        assert_eq!(source.to_string(), INTERNAL_ERROR_MESSAGE);
        let db = source.source().expect("original DbErr");
        assert!(db.to_string().contains("SELECT"));
    }

    #[test]
    fn partial_eq_ignores_source() {
        let left = RestError::internal(sea_orm::DbErr::Custom("one".into()));
        let right = RestError::internal(sea_orm::DbErr::Custom("two".into()));
        assert_eq!(left, right);
        assert_ne!(
            left,
            RestError::new(StatusCode::NOT_FOUND, INTERNAL_ERROR_MESSAGE)
        );
    }
}
