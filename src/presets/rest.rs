//! Generic REST error map. Covers `cgp-orm` sources and `DbErr`.
//!
//! [`sea_orm::DbErr`] and unexpected `String` / `&str` are mapped by
//! [`crate::error::RaiseRestInternal`] to HTTP 500 with a fixed
//! `"internal error"` client message. The original is on
//! [`crate::error::RestError`]'s [`Error::source`](core::error::Error::source).
//! The raiser keys are `open`: an application that wants the raw text on
//! a private network overrides that one key.
//!
//! ## How to return 404
//!
//! [`cgp_orm::CanGetResource::get`] returns `Ok(None)` when a row is
//! missing. That is success: a handler that only matches
//! [`crate::error::RestError`] will answer 200 with an empty body.
//! [`cgp_orm::error::ErrEntityNotFound`] is already mapped to 404 here,
//! but the default provider never raises it. Leave `get` as `Option`.
//! To answer 404, use the opt-in helper next to `CanGetResource` in
//! `cgp-orm` (`get_or_not_found`), which raises `ErrEntityNotFound`.
//! This crate does not authenticate, and a live `db_conn` behind a route
//! is full CRUD; there is no `CanAuthorize`.

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
    RaiseRestBadRequest, RaiseRestInternal, RaiseRestInternalDisplay, RaiseRestNotFound, RestError,
    WrapRestError,
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
        ]: RaiseRestInternal,
        @ErrorRaiserComponent.<'a> &'a str: RaiseRestInternal,
        @ErrorRaiserComponent.[
            ErrMissingPrimaryKey,
            ErrUnsupportedKeyArity,
            ErrConflictFetchFailed,
            ErrUniqueConflict,
        ]: RaiseRestInternalDisplay,
        @ErrorRaiserComponent.ErrEntityNotFound: RaiseRestNotFound,
        @ErrorRaiserComponent.[
            ErrIdentityEmpty,
            ErrMissingOrdering,
        ]: RaiseRestBadRequest,
        @ErrorRaiserComponent.<Column> ErrIdentityFieldUnset<Column>: RaiseRestBadRequest,
        @ErrorRaiserComponent.<Column> ErrPrimaryKeyUnset<Column>: RaiseRestBadRequest,
    }
}
