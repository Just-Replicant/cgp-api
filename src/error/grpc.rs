//! gRPC status raisers. Application policy — not part of `cgp-orm`.
//!
//! [`RaiseGrpcInternal`] is registered for [`sea_orm::DbErr`], `String`, and
//! `&str`. [`tonic::Status::message`] is what the client sees, so it is a
//! fixed `"internal error"` — never the Display of a [`sea_orm::DbErr`].
//! The original is attached with [`Status::set_source`](tonic::Status::set_source)
//! for in-process inspection. This crate does not add a logger or a
//! debug-errors feature. The raiser keys are already `open`; an application
//! that wants the raw text on a private network overrides that one key.

use core::error::Error;
use core::fmt::Display;
use std::sync::Arc;

use cgp_fork::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;
use cgp_orm::error::ErrEntityNotFound;
use tonic::Status;

use crate::error::{INTERNAL_ERROR_MESSAGE, UnexpectedMessage};

/// Map [`ErrEntityNotFound`] to `NOT_FOUND`.
pub struct RaiseGrpcNotFound;

#[cgp_impl(RaiseGrpcNotFound)]
#[use_type(HasErrorType.{Error = Status})]
impl ErrorRaiser<ErrEntityNotFound> {
    fn raise_error(e: ErrEntityNotFound) -> Status {
        let mut status = Status::not_found(e.to_string());
        status.set_source(Arc::new(e));
        status
    }
}

/// Map [`sea_orm::DbErr`], `String`, and `&str` to `INTERNAL` with a fixed
/// [`INTERNAL_ERROR_MESSAGE`].
///
/// [`Status::message`] is the gRPC client body, so it stays generic. The
/// original is [`Error::source`]. Typed internals that are safe to show
/// use [`RaiseGrpcInternalDisplay`].
///
/// To send the raw text to a client (for example on a private network),
/// override this raiser key on a child table — the keys are `open`.
pub struct RaiseGrpcInternal;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, sea_orm::DbErr> for RaiseGrpcInternal
where
    Context: HasErrorType<Error = Status>,
{
    fn raise_error(e: sea_orm::DbErr) -> Status {
        let mut status = Status::internal(INTERNAL_ERROR_MESSAGE);
        status.set_source(Arc::new(e));
        status
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, String> for RaiseGrpcInternal
where
    Context: HasErrorType<Error = Status>,
{
    fn raise_error(e: String) -> Status {
        let mut status = Status::internal(INTERNAL_ERROR_MESSAGE);
        status.set_source(Arc::new(UnexpectedMessage(e)));
        status
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, &str> for RaiseGrpcInternal
where
    Context: HasErrorType<Error = Status>,
{
    fn raise_error(e: &str) -> Status {
        let mut status = Status::internal(INTERNAL_ERROR_MESSAGE);
        status.set_source(Arc::new(UnexpectedMessage(e.to_owned())));
        status
    }
}

/// Map a [`Display`] + [`Error`] source to `INTERNAL`, using its Display as
/// [`Status::message`].
///
/// Used for typed library failures that do not contain SQL. For
/// [`sea_orm::DbErr`] and unexpected strings, use [`RaiseGrpcInternal`].
pub struct RaiseGrpcInternalDisplay;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseGrpcInternalDisplay
where
    Context: HasErrorType<Error = Status>,
    E: Display + Error + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Status {
        let mut status = Status::internal(e.to_string());
        status.set_source(Arc::new(e));
        status
    }
}

/// Map any [`Display`] source to `INVALID_ARGUMENT`.
pub struct RaiseGrpcInvalidArgument;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseGrpcInvalidArgument
where
    Context: HasErrorType<Error = Status>,
    E: Display + Error + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Status {
        let mut status = Status::invalid_argument(e.to_string());
        status.set_source(Arc::new(e));
        status
    }
}

/// Prefix `{detail}: {message}` on the *client* message ([`Status::message`])
/// and keep the original status code and source chain.
///
/// Uses [`Status::message`], not [`Display`], so wrapping cannot leak a
/// [`sea_orm::DbErr`] that was attached as source (Display of `Status`
/// includes the source).
pub struct WrapGrpcStatus;

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for WrapGrpcStatus
where
    Context: HasErrorType<Error = Status>,
    Detail: Display,
{
    fn wrap_error(error: Status, detail: Detail) -> Status {
        let mut status = Status::new(error.code(), format!("{detail}: {}", error.message()));
        status.set_source(Arc::new(error));
        status
    }
}

#[cfg(test)]
mod tests {
    use core::error::Error;

    use cgp_fork::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp_fork::prelude::*;
    use cgp_orm::error::{ErrEntityNotFound, ErrUniqueConflict};
    use tonic::{Code, Status};

    use crate::error::{INTERNAL_ERROR_MESSAGE, UnexpectedMessage};
    use crate::presets::GrpcErrors;

    struct ErrorApp;

    delegate_components! {
        #[unchecked]
        ErrorApp {
            ErrorTypeProviderComponent:
                <GrpcErrors as DelegateComponent<ErrorTypeProviderComponent>>::Delegate,
            ErrorRaiserComponent:
                <GrpcErrors as DelegateComponent<ErrorRaiserComponent>>::Delegate,
            ErrorWrapperComponent:
                <GrpcErrors as DelegateComponent<ErrorWrapperComponent>>::Delegate,
        }
    }

    #[test]
    fn db_err_is_opaque_to_the_client() {
        let sql = "SELECT * FROM secrets; syntax error at or near \"SELECT\"";
        let db = sea_orm::DbErr::Custom(sql.into());
        let error: Status = ErrorApp::raise_error(db);
        assert_eq!(error.code(), Code::Internal);
        assert_eq!(error.message(), INTERNAL_ERROR_MESSAGE);
        assert!(
            !error.message().contains("SELECT"),
            "Status message must not contain SQL: {}",
            error.message()
        );
        let source = error.source().expect("original DbErr");
        assert!(source.to_string().contains("SELECT"));
        assert!(source.downcast_ref::<sea_orm::DbErr>().is_some());
    }

    #[test]
    fn unexpected_strings_are_opaque_to_the_client() {
        let from_str: Status = ErrorApp::raise_error("sql leaked");
        assert_eq!(from_str.code(), Code::Internal);
        assert_eq!(from_str.message(), INTERNAL_ERROR_MESSAGE);
        let source = from_str.source().expect("original message");
        assert_eq!(source.to_string(), "sql leaked");
        assert_eq!(
            source.downcast_ref::<UnexpectedMessage>(),
            Some(&UnexpectedMessage("sql leaked".into()))
        );

        let from_string: Status = ErrorApp::raise_error(String::from("sql leaked"));
        assert_eq!(from_string.message(), INTERNAL_ERROR_MESSAGE);
        assert_eq!(
            from_string.source().map(ToString::to_string).as_deref(),
            Some("sql leaked")
        );
    }

    #[test]
    fn unique_conflict_uses_its_display_as_the_message() {
        let error: Status = ErrorApp::raise_error(ErrUniqueConflict);
        assert_eq!(error.code(), Code::Internal);
        assert_eq!(error.message(), ErrUniqueConflict.to_string());
    }

    #[test]
    fn not_found_uses_the_typed_message() {
        let not_found = ErrEntityNotFound::of::<u32>();
        let entity_name = not_found.entity_name;
        let error: Status = ErrorApp::raise_error(not_found);
        assert_eq!(error.code(), Code::NotFound);
        assert_eq!(error.message(), format!("entity {entity_name} not found"));
    }

    #[test]
    fn wrap_uses_the_client_message_and_does_not_leak_db_err() {
        let db = sea_orm::DbErr::Custom("SELECT leaked".into());
        let error: Status = ErrorApp::raise_error(db);
        let wrapped = ErrorApp::wrap_error(error, "load user");
        assert_eq!(wrapped.code(), Code::Internal);
        assert_eq!(
            wrapped.message(),
            format!("load user: {INTERNAL_ERROR_MESSAGE}")
        );
        assert!(
            !wrapped.message().contains("SELECT"),
            "wrapped Status message leaked SQL: {}",
            wrapped.message()
        );
        // Display of Status includes source; the client body is `message()`.
        let displayed = wrapped.to_string();
        // The wrapped Status's source is the previous Status, whose Display
        // (not its message) may mention the DbErr. That is in-process only.
        let _ = displayed;
        let source = wrapped.source().expect("previous Status");
        // The previous Status's message is generic; walk to the DbErr.
        let mut current: Option<&dyn Error> = Some(source);
        let mut found_sql = false;
        while let Some(error) = current {
            if error.to_string().contains("SELECT") {
                found_sql = true;
                break;
            }
            if let Some(db) = error.downcast_ref::<sea_orm::DbErr>() {
                found_sql = db.to_string().contains("SELECT");
                break;
            }
            current = error.source();
        }
        assert!(found_sql, "in-process chain should still hold the DbErr");
    }
}
