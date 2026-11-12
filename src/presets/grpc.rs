//! Generic gRPC error map. Covers `cgp-orm` sources and `DbErr`.

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
    RaiseGrpcInternal, RaiseGrpcInvalidArgument, RaiseGrpcNotFound, WrapGrpcStatus,
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
            ErrMissingPrimaryKey,
            ErrUnsupportedKeyArity,
            ErrConflictFetchFailed,
            ErrUniqueConflict,
        ]: RaiseGrpcInternal,
        @ErrorRaiserComponent.<'a> &'a str: RaiseGrpcInternal,
        @ErrorRaiserComponent.ErrEntityNotFound: RaiseGrpcNotFound,
        @ErrorRaiserComponent.[
            ErrIdentityEmpty,
            ErrMissingOrdering,
        ]: RaiseGrpcInvalidArgument,
        @ErrorRaiserComponent.<Column> ErrIdentityFieldUnset<Column>: RaiseGrpcInvalidArgument,
        @ErrorRaiserComponent.<Column> ErrPrimaryKeyUnset<Column>: RaiseGrpcInvalidArgument,
    }
}
