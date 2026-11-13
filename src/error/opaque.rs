//! Shared client-facing internal-error text and the in-process wrapper for
//! unexpected `String` / `&str` sources.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// Client-facing message for [`super::RaiseRestInternal`] and
/// [`super::RaiseGrpcInternal`].
///
/// Used at the REST and gRPC boundaries for [`sea_orm::DbErr`] and for
/// unexpected `String` / `&str`. In-process [`super::ApiError`] still
/// carries the full original.
pub const INTERNAL_ERROR_MESSAGE: &str = "internal error";

/// An unexpected `String` or `&str` raised as an internal error.
///
/// These values are not [`Error`] types on their own. The original text is
/// stored here so [`Error::source`] on the transport error can still expose
/// it in-process. It is never used as the client body by the internal
/// raisers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnexpectedMessage(pub String);

impl Display for UnexpectedMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for UnexpectedMessage {}
