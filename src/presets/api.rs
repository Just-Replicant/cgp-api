//! Generic in-process error map. Covers `cgp-orm` sources and `DbErr`.
//!
//! Applications with extra source errors write their own table.
//! Embed [`crate::error::ApiError`] and raise library sources with
//! [`crate::error::RaiseViaApiError`]. Raiser keys are `open` so a
//! child table can add sources. The wrapper is one component;
//! replace `ErrorWrapperComponent` to attach details differently.

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

use crate::error::{
    ApiError, RaiseApiMessage, RaiseIdentityFieldUnset, RaisePrimaryKeyUnset, RaiseViaApiError,
    WrapApiError,
};

/// Shared in-process error table. Contexts forward the three error components
/// to this table's delegates, or inherit them through
/// [`super::ApiResourceStack`].
pub struct ApiErrors;

delegate_components! {
    #[unchecked]
    ApiErrors {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseType<ApiError>,
        ErrorWrapperComponent: WrapApiError,
        @ErrorRaiserComponent.ApiError: ReturnError,
        @ErrorRaiserComponent.core::convert::Infallible: RaiseInfallible,
        @ErrorRaiserComponent.[
            ::sea_orm::DbErr,
            ErrIdentityEmpty,
            ErrMissingPrimaryKey,
            ErrUnsupportedKeyArity,
            ErrMissingOrdering,
            ErrConflictFetchFailed,
            ErrEntityNotFound,
            ErrUniqueConflict,
        ]: RaiseViaApiError,
        @ErrorRaiserComponent.String: RaiseApiMessage,
        @ErrorRaiserComponent.<'a> &'a str: RaiseApiMessage,
        @ErrorRaiserComponent.<Column> ErrIdentityFieldUnset<Column>: RaiseIdentityFieldUnset,
        @ErrorRaiserComponent.<Column> ErrPrimaryKeyUnset<Column>: RaisePrimaryKeyUnset,
    }
}
