//! Generic gRPC error map. Covers `cgp-orm` sources and `DbErr`.
//!
//! [`sea_orm::DbErr`] and unexpected `String` / `&str` are mapped by
//! [`crate::error::RaiseGrpcInternal`] to `INTERNAL` with a fixed
//! `"internal error"` message. [`tonic::Status::message`] is what the
//! client sees, so it stays generic. The raiser keys are `open`.
//!

use cgp_fork::core::error::{
    ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use cgp_fork::extra::error::{RaiseInfallible, ReturnError};
use cgp_fork::prelude::*;
use cgp_orm::error::{
    ErrConflictFetchFailed, ErrEntityNotFound, ErrIdentityEmpty, ErrIdentityFieldUnset,
    ErrMissingOrdering, ErrMissingPrimaryKey, ErrPrimaryKeyUnset, ErrUniqueConflict,
    ErrUnsupportedKeyArity,
};
use tonic::Status;

use crate::error::{
    RaiseGrpcInternal, RaiseGrpcInternalDisplay, RaiseGrpcInvalidArgument, RaiseGrpcNotFound,
    WrapGrpcStatus,
};

/// Shared gRPC error table.
pub struct GrpcErrors;

delegate_components! {
    #[unchecked]
    GrpcErrors {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseType<Status>,
        ErrorWrapperComponent: WrapGrpcStatus,
        @ErrorRaiserComponent.tonic::Status: ReturnError,
        @ErrorRaiserComponent.core::convert::Infallible: RaiseInfallible,
        @ErrorRaiserComponent.[
            ::sea_orm::DbErr,
            String,
        ]: RaiseGrpcInternal,
        @ErrorRaiserComponent.<'a> &'a str: RaiseGrpcInternal,
        @ErrorRaiserComponent.[
            ErrMissingPrimaryKey,
            ErrUnsupportedKeyArity,
            ErrConflictFetchFailed,
            ErrUniqueConflict,
        ]: RaiseGrpcInternalDisplay,
        @ErrorRaiserComponent.ErrEntityNotFound: RaiseGrpcNotFound,
        @ErrorRaiserComponent.[
            ErrIdentityEmpty,
            ErrMissingOrdering,
        ]: RaiseGrpcInvalidArgument,
        @ErrorRaiserComponent.<Column> ErrIdentityFieldUnset<Column>: RaiseGrpcInvalidArgument,
        @ErrorRaiserComponent.<Column> ErrPrimaryKeyUnset<Column>: RaiseGrpcInvalidArgument,
    }
}
