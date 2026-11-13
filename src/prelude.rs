//! Convenient re-exports for application consumers.

pub use cgp_fork::prelude::*;
pub use cgp_orm::prelude::*;

pub use crate::accessors::HasEntityName;
pub use crate::codec::{
    ApplyUpdateInput, ApplyUpdateInputComponent, DecodeCreateInput, DecodeCreateInputComponent,
    EncodeOutput, EncodeOutputComponent,
};
#[cfg(feature = "rest")]
pub use crate::error::RestError;
#[cfg(feature = "thiserror")]
pub use crate::error::{
    ApiError, RaiseApiMessage, RaiseIdentityFieldUnset, RaisePrimaryKeyUnset, RaiseViaApiError,
    WrapApiError, WrapDetail,
};
#[cfg(any(feature = "rest", feature = "grpc"))]
pub use crate::error::{INTERNAL_ERROR_MESSAGE, UnexpectedMessage};
#[cfg(feature = "grpc")]
pub use crate::error::{
    RaiseGrpcInternal, RaiseGrpcInternalDisplay, RaiseGrpcInvalidArgument, RaiseGrpcNotFound,
};
#[cfg(feature = "rest")]
pub use crate::error::{
    RaiseRestBadRequest, RaiseRestInternal, RaiseRestInternalDisplay, RaiseRestNotFound,
};
#[cfg(feature = "thiserror")]
pub use crate::presets::{ApiCrudStack, ApiErrors, ApiLookupStack, ApiResourceStack};
#[cfg(feature = "grpc")]
pub use crate::presets::{GrpcCrudStack, GrpcErrors, GrpcLookupStack, GrpcResourceStack};
#[cfg(feature = "rest")]
pub use crate::presets::{RestCrudStack, RestErrors, RestLookupStack, RestResourceStack};
pub use crate::transport::{
    PageMeta, ResourceResponse, ResourceResponseList, response_from, response_from_page,
    response_list_from,
};
