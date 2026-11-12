//! Generic REST error map. Covers `cgp-orm` sources and `DbErr`.

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
    RaiseRestBadRequest, RaiseRestInternal, RaiseRestNotFound, RestError, WrapRestError,
};

/// Shared REST error table.
pub struct RestErrors;

delegate_components! {
    #[unchecked]
    RestErrors {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseType<RestError>,
        ErrorWrapperComponent: WrapRestError,
        @ErrorRaiserComponent.RestError: ReturnError,
        @ErrorRaiserComponent.core::convert::Infallible: RaiseInfallible,
        @ErrorRaiserComponent.[
            ::sea_orm::DbErr,
            String,
            ErrMissingPrimaryKey,
            ErrUnsupportedKeyArity,
            ErrConflictFetchFailed,
            ErrUniqueConflict,
        ]: RaiseRestInternal,
        @ErrorRaiserComponent.<'a> &'a str: RaiseRestInternal,
        @ErrorRaiserComponent.ErrEntityNotFound: RaiseRestNotFound,
        @ErrorRaiserComponent.[
            ErrIdentityEmpty,
            ErrMissingOrdering,
        ]: RaiseRestBadRequest,
        @ErrorRaiserComponent.<Column> ErrIdentityFieldUnset<Column>: RaiseRestBadRequest,
        @ErrorRaiserComponent.<Column> ErrPrimaryKeyUnset<Column>: RaiseRestBadRequest,
    }
}
