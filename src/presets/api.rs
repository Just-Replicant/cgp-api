//! Generic in-process error map. Covers `cgp-orm` sources and `DbErr`.
//!
//! # Extending with an application error
//!
//! The extension path is `open ErrorRaiserComponent`, an application
//! enum that **embeds** [`crate::error::ApiError`] with `#[from]`, and
//! [`crate::error::RaiseViaApiError`] for library sources. It is not an
//! `anyhow` table, and it is not a copy of this table's source list.
//!
//! A child table opens the raiser, sets `Error` to the application enum,
//! implements [`crate::error::WrapDetail`] so [`crate::error::WrapApiError`]
//! still wraps, and registers `ReturnError` for the enum and `RaiseFrom`
//! for the new source — without repeating this table's source list.
//! Library sources stay on this table: `RaiseViaApiError` already
//! accepts `Context::Error: From<ApiError>`, which the embedded `#[from]`
//! provides, so a context forwards `ErrorRaiserComponent` to this
//! table's delegate. A `RedirectLookup` consults one table, so the new
//! source and the library sources are two wirings, not one copied table.
//! See the `child_table_raises_an_app_error` test in `src/error/api.rs`.
//!
//! The wrapper is one component; replace `ErrorWrapperComponent` to
//! attach details differently.

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
/// [`ApiErrorPolicy`].
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

// Forwards the three error components to `ApiErrors`.
// Compose with a cgp-orm stack; this policy does not wrap an ORM parent.
cgp_preset! {
    ApiErrorPolicy {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::ApiErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::ApiErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::ApiErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}
