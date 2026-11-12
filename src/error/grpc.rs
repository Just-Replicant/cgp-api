//! gRPC status raisers. Application policy — not part of `cgp-orm`.

use core::fmt::Display;

use cgp_fork::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;
use cgp_orm::error::ErrEntityNotFound;
use tonic::Status;

/// Map [`ErrEntityNotFound`] to `NOT_FOUND`.
pub struct RaiseGrpcNotFound;

#[cgp_impl(RaiseGrpcNotFound)]
#[use_type(HasErrorType.{Error = Status})]
impl ErrorRaiser<ErrEntityNotFound> {
    fn raise_error(e: ErrEntityNotFound) -> Status {
        Status::not_found(e.to_string())
    }
}

/// Map any [`Display`] source to `INTERNAL`.
pub struct RaiseGrpcInternal;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseGrpcInternal
where
    Context: HasErrorType<Error = Status>,
    E: Display,
{
    fn raise_error(e: E) -> Status {
        Status::internal(e.to_string())
    }
}

/// Map any [`Display`] source to `INVALID_ARGUMENT`.
pub struct RaiseGrpcInvalidArgument;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, E> ErrorRaiser<Context, E> for RaiseGrpcInvalidArgument
where
    Context: HasErrorType<Error = Status>,
    E: Display,
{
    fn raise_error(e: E) -> Status {
        Status::invalid_argument(e.to_string())
    }
}

/// Prefix `{detail}: {error}` and keep the original status code.
pub struct WrapGrpcStatus;

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for WrapGrpcStatus
where
    Context: HasErrorType<Error = Status>,
    Detail: Display,
{
    fn wrap_error(error: Status, detail: Detail) -> Status {
        Status::new(error.code(), format!("{detail}: {error}"))
    }
}
