//! In-process error type and raisers.
//!
//! [`ApiError`] covers the sources this crate already maps: `cgp-orm`
//! errors, [`sea_orm::DbErr`], and ad-hoc messages. An application with
//! further sources defines its own enum, embeds [`ApiError`] with
//! `#[from]`, and raises the library sources through [`RaiseViaApiError`].

use core::fmt::{Debug, Display};

use cgp_fork::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;
use cgp_orm::error::{ErrIdentityFieldUnset, ErrPrimaryKeyUnset};
use thiserror::Error;

/// In-process failure for a context that uses [`crate::presets::ApiErrors`].
///
/// REST and gRPC keep their own error types. This enum is exhaustive so
/// callers can match on the repository failure.
#[derive(Debug, Error)]
pub enum ApiError {
    /// A SeaORM failure. Boxed so the enum stays small.
    #[error(transparent)]
    Db(Box<sea_orm::DbErr>),

    /// [`cgp_orm::error::ErrEntityNotFound`].
    #[error(transparent)]
    NotFound(#[from] cgp_orm::error::ErrEntityNotFound),

    /// [`cgp_orm::error::ErrIdentityEmpty`].
    #[error(transparent)]
    IdentityEmpty(#[from] cgp_orm::error::ErrIdentityEmpty),

    /// [`cgp_orm::error::ErrMissingPrimaryKey`].
    #[error(transparent)]
    MissingPrimaryKey(#[from] cgp_orm::error::ErrMissingPrimaryKey),

    /// [`cgp_orm::error::ErrUnsupportedKeyArity`].
    #[error(transparent)]
    UnsupportedKeyArity(#[from] cgp_orm::error::ErrUnsupportedKeyArity),

    /// [`cgp_orm::error::ErrMissingOrdering`].
    #[error(transparent)]
    MissingOrdering(#[from] cgp_orm::error::ErrMissingOrdering),

    /// [`cgp_orm::error::ErrConflictFetchFailed`].
    #[error(transparent)]
    ConflictFetchFailed(#[from] cgp_orm::error::ErrConflictFetchFailed),

    /// [`cgp_orm::error::ErrUniqueConflict`].
    #[error(transparent)]
    UniqueConflict(#[from] cgp_orm::error::ErrUniqueConflict),

    /// [`ErrIdentityFieldUnset`], with the column rendered via [`Debug`].
    #[error("identity field {column} is not set")]
    IdentityFieldUnset {
        /// Debug formatting of the unset column.
        column: String,
    },

    /// [`ErrPrimaryKeyUnset`], with the column rendered via [`Debug`].
    #[error("primary-key field {column} is not set")]
    PrimaryKeyUnset {
        /// Debug formatting of the unset column.
        column: String,
    },

    /// An ad-hoc `String` or `&str`. The value is not a [`core::error::Error`].
    #[error("{0}")]
    Message(String),

    /// A detail attached by [`WrapApiError`]. The previous error is [`source`](core::error::Error::source).
    #[error("{detail}: {source}")]
    Wrapped {
        /// Context added by the wrapper.
        detail: String,
        /// The error that was wrapped.
        #[source]
        source: Box<ApiError>,
    },
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        Self::Db(Box::new(error))
    }
}

/// Context error that can nest a display detail and keep `self` as the source.
///
/// [`ApiError`] implements this. An application enum that embeds [`ApiError`]
/// implements it too, so [`WrapApiError`] can serve both.
pub trait WrapDetail: Sized {
    /// Return `self` with `detail` attached.
    fn wrap_detail(self, detail: impl Display) -> Self;
}

impl WrapDetail for ApiError {
    fn wrap_detail(self, detail: impl Display) -> Self {
        Self::Wrapped {
            detail: detail.to_string(),
            source: Box::new(self),
        }
    }
}

/// Raise a source that converts into [`ApiError`], then into the context error.
///
/// `Context::Error` is [`ApiError`], or an application enum with
/// `From<ApiError>`. Each library source has its own `From` impl on
/// [`ApiError`] (`#[from]`). [`RaiseViaApiError`] applies that conversion
/// and then `From<ApiError>` for the context error. Register it on the
/// library sources; register [`cgp_fork::extra::error::RaiseFrom`] on
/// sources that convert directly into the application enum.
pub struct RaiseViaApiError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseViaApiError
where
    Context: HasErrorType,
    Context::Error: From<ApiError>,
    ApiError: From<E>,
{
    fn raise_error(e: E) -> Context::Error {
        Context::Error::from(ApiError::from(e))
    }
}

/// Raise a [`Display`] value as [`ApiError::Message`].
///
/// The original value is not kept. The map registers this for `String` and
/// `&str` only.
pub struct RaiseApiMessage;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseApiMessage
where
    Context: HasErrorType,
    Context::Error: From<ApiError>,
    E: Display,
{
    fn raise_error(e: E) -> Context::Error {
        ApiError::Message(e.to_string()).into()
    }
}

/// Raise [`ErrIdentityFieldUnset`] by formatting the column with [`Debug`].
pub struct RaiseIdentityFieldUnset;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, Column> ErrorRaiser<Context, ErrIdentityFieldUnset<Column>>
    for RaiseIdentityFieldUnset
where
    Context: HasErrorType,
    Context::Error: From<ApiError>,
    Column: Debug,
{
    fn raise_error(e: ErrIdentityFieldUnset<Column>) -> Context::Error {
        let ErrIdentityFieldUnset { column } = e;
        ApiError::IdentityFieldUnset {
            column: format!("{column:?}"),
        }
        .into()
    }
}

/// Raise [`ErrPrimaryKeyUnset`] by formatting the column with [`Debug`].
pub struct RaisePrimaryKeyUnset;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, Column> ErrorRaiser<Context, ErrPrimaryKeyUnset<Column>> for RaisePrimaryKeyUnset
where
    Context: HasErrorType,
    Context::Error: From<ApiError>,
    Column: Debug,
{
    fn raise_error(e: ErrPrimaryKeyUnset<Column>) -> Context::Error {
        let ErrPrimaryKeyUnset { column } = e;
        ApiError::PrimaryKeyUnset {
            column: format!("{column:?}"),
        }
        .into()
    }
}

/// Attach a [`Display`] detail through [`WrapDetail`].
pub struct WrapApiError;

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for WrapApiError
where
    Context: HasErrorType,
    Context::Error: WrapDetail,
    Detail: Display,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        error.wrap_detail(detail)
    }
}

#[cfg(test)]
mod tests {
    use core::error::Error;

    use cgp_fork::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp_fork::prelude::*;
    use cgp_orm::error::{ErrEntityNotFound, ErrIdentityFieldUnset, ErrPrimaryKeyUnset};

    use super::ApiError;
    use crate::presets::ApiErrors;

    struct ErrorApp;

    delegate_components! {
        #[unchecked]
        ErrorApp {
            ErrorTypeProviderComponent:
                <ApiErrors as DelegateComponent<ErrorTypeProviderComponent>>::Delegate,
            ErrorRaiserComponent:
                <ApiErrors as DelegateComponent<ErrorRaiserComponent>>::Delegate,
            ErrorWrapperComponent:
                <ApiErrors as DelegateComponent<ErrorWrapperComponent>>::Delegate,
        }
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn api_error_is_send_and_sync() {
        assert_send_sync::<ApiError>();
    }

    #[test]
    fn not_found_stays_a_variant() {
        let not_found = ErrEntityNotFound::of::<u32>();
        let entity_name = not_found.entity_name;
        let error = ErrorApp::raise_error(not_found);
        assert_eq!(error.to_string(), format!("entity {entity_name} not found"));
        // Transparent: `source()` forwards the inner error's cause.
        // `ErrEntityNotFound` has none, so the chain stops here.
        assert!(error.source().is_none());
        assert!(matches!(
            error,
            ApiError::NotFound(ErrEntityNotFound { entity_name: name }) if name == entity_name
        ));
    }

    #[test]
    fn messages_drop_the_source() {
        let from_str = ErrorApp::raise_error("missing");
        assert!(matches!(&from_str, ApiError::Message(message) if message == "missing"));
        assert!(from_str.source().is_none());

        let from_string = ErrorApp::raise_error(String::from("missing"));
        assert!(matches!(&from_string, ApiError::Message(message) if message == "missing"));
        assert!(from_string.source().is_none());
    }

    #[test]
    fn column_errors_render_debug() {
        #[derive(Debug)]
        struct Email;

        let identity = ErrorApp::raise_error(ErrIdentityFieldUnset { column: Email });
        assert_eq!(identity.to_string(), "identity field Email is not set");
        assert!(matches!(
            identity,
            ApiError::IdentityFieldUnset { column } if column == "Email"
        ));

        let primary_key = ErrorApp::raise_error(ErrPrimaryKeyUnset { column: Email });
        assert_eq!(
            primary_key.to_string(),
            "primary-key field Email is not set"
        );
    }

    #[test]
    fn wrap_nests_the_previous_error() {
        let not_found = ErrEntityNotFound::of::<u32>();
        let entity_name = not_found.entity_name;
        let error = ErrorApp::wrap_error(ApiError::from(not_found), "load user");
        let expected = format!("entity {entity_name} not found");
        assert_eq!(error.to_string(), format!("load user: {expected}"));
        assert_eq!(
            error.source().map(ToString::to_string).as_deref(),
            Some(expected.as_str())
        );
        let ApiError::Wrapped { detail, source } = error else {
            panic!("expected wrapped error");
        };
        assert_eq!(detail, "load user");
        assert!(matches!(
            source.as_ref(),
            ApiError::NotFound(ErrEntityNotFound { entity_name: name }) if *name == entity_name
        ));
    }
    #[test]
    fn db_err_keeps_the_full_display_in_process() {
        let sql = "SELECT * FROM secrets; syntax error at or near \"SELECT\"";
        let db = sea_orm::DbErr::Custom(sql.into());
        let error = ErrorApp::raise_error(db);
        assert!(
            error.to_string().contains("SELECT"),
            "in-process ApiError must keep the SQL, got {error}"
        );
        assert!(matches!(error, ApiError::Db(_)));
    }

    #[derive(Debug, thiserror::Error, PartialEq, Eq)]
    #[error("quota exceeded")]
    struct QuotaExceeded;

    #[derive(Debug, thiserror::Error)]
    enum AppError {
        #[error(transparent)]
        Api(#[from] ApiError),
        #[error(transparent)]
        Quota(#[from] QuotaExceeded),
    }

    impl crate::error::WrapDetail for AppError {
        fn wrap_detail(self, detail: impl core::fmt::Display) -> Self {
            match self {
                AppError::Api(api) => AppError::Api(api.wrap_detail(detail)),
                other => AppError::Api(ApiError::Message(other.to_string()).wrap_detail(detail)),
            }
        }
    }

    /// Child table: one app source, no copy of [`crate::presets::ApiErrors`].
    struct AppErrors;

    delegate_components! {
        #[unchecked]
        AppErrors {
            open ErrorRaiserComponent;

            ErrorTypeProviderComponent: UseType<AppError>,
            ErrorWrapperComponent: crate::error::WrapApiError,
            @ErrorRaiserComponent.AppError: cgp_fork::extra::error::ReturnError,
            @ErrorRaiserComponent.QuotaExceeded: cgp_fork::extra::error::RaiseFrom,
        }
    }

    struct AppContext;

    delegate_components! {
        #[unchecked]
        AppContext {
            ErrorTypeProviderComponent:
                <AppErrors as DelegateComponent<ErrorTypeProviderComponent>>::Delegate,
            ErrorRaiserComponent:
                <AppErrors as DelegateComponent<ErrorRaiserComponent>>::Delegate,
            ErrorWrapperComponent:
                <AppErrors as DelegateComponent<ErrorWrapperComponent>>::Delegate,
        }
    }

    /// Same `AppError` (embeds `ApiError`), but the raiser is `ApiErrors`
    /// itself — library sources go through `RaiseViaApiError` without a
    /// copy of that table.
    struct EmbeddedApp;

    delegate_components! {
        #[unchecked]
        EmbeddedApp {
            ErrorTypeProviderComponent: UseType<AppError>,
            ErrorRaiserComponent:
                <ApiErrors as DelegateComponent<ErrorRaiserComponent>>::Delegate,
            ErrorWrapperComponent:
                <ApiErrors as DelegateComponent<ErrorWrapperComponent>>::Delegate,
        }
    }

    #[test]
    fn child_table_raises_an_app_error_without_copying_api_errors() {
        // The child table lists only the new source. It does not repeat
        // the ApiErrors key list.
        let error = AppContext::raise_error(QuotaExceeded);
        assert!(matches!(error, AppError::Quota(QuotaExceeded)));
        assert_eq!(error.to_string(), "quota exceeded");

        // Library sources: RaiseViaApiError already accepts
        // `Context::Error: From<ApiError>`, which the embedded `#[from]`
        // provides. Forward ErrorRaiserComponent to ApiErrors — do not
        // copy its source list. A RedirectLookup consults one table, so
        // this is a second wiring, not a single table that holds both.
        let not_found = ErrEntityNotFound::of::<u32>();
        let entity_name = not_found.entity_name;
        let error = EmbeddedApp::raise_error(not_found);
        assert_eq!(error.to_string(), format!("entity {entity_name} not found"));
        assert!(matches!(
            error,
            AppError::Api(ApiError::NotFound(ErrEntityNotFound { entity_name: name }))
                if name == entity_name
        ));
    }
}
