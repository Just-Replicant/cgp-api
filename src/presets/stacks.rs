//! Presets that compose a `cgp-orm` stack with a generic error map.
//!
//! A consumer fills in the entity, codecs, and identity:
//!
//! ```ignore
//! cgp_preset! {
//!     PostAppPreset: ApiResourceStack {
//!         EntityTypeProviderComponent: UseType<post::Entity>,
//!         CreateInputTypeProviderComponent: UseType<CreatePost>,
//!         UpdateInputTypeProviderComponent: UseType<UpdatePost>,
//!         OutputTypeProviderComponent: UseType<PostOutput>,
//!         // codecs, UniqueColumnsGetter, EntityNameGetter, hooks
//!     }
//! }
//! ```

#![allow(missing_docs)] // generated preset modules

use cgp_fork::prelude::*;

// --- in-process (thiserror) ----------------------------------------------

#[cfg(feature = "thiserror")]
cgp_preset! {
    ApiLookupStack: cgp_orm::presets::RepositoryLookupStack {
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

#[cfg(feature = "thiserror")]
cgp_preset! {
    ApiCrudStack: cgp_orm::presets::RepositoryCrudStack {
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

#[cfg(feature = "thiserror")]
cgp_preset! {
    ApiResourceStack: cgp_orm::presets::RepositoryCrudStack + cgp_orm::presets::ResourceServiceStack {
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

// --- grpc ----------------------------------------------------------------

#[cfg(feature = "grpc")]
cgp_preset! {
    GrpcLookupStack: cgp_orm::presets::RepositoryLookupStack {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}

#[cfg(feature = "grpc")]
cgp_preset! {
    GrpcCrudStack: cgp_orm::presets::RepositoryCrudStack {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}

#[cfg(feature = "grpc")]
cgp_preset! {
    GrpcResourceStack: cgp_orm::presets::RepositoryCrudStack + cgp_orm::presets::ResourceServiceStack {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::GrpcErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}

// --- rest ----------------------------------------------------------------

#[cfg(feature = "rest")]
cgp_preset! {
    RestLookupStack: cgp_orm::presets::RepositoryLookupStack {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}

#[cfg(feature = "rest")]
cgp_preset! {
    RestCrudStack: cgp_orm::presets::RepositoryCrudStack {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}

#[cfg(feature = "rest")]
cgp_preset! {
    RestResourceStack: cgp_orm::presets::RepositoryCrudStack + cgp_orm::presets::ResourceServiceStack {
        ::cgp_fork::core::error::ErrorTypeProviderComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorTypeProviderComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorRaiserComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorRaiserComponent,
            >>::Delegate,
        ::cgp_fork::core::error::ErrorWrapperComponent:
            <cgp_api::presets::RestErrors as DelegateComponent<
                ::cgp_fork::core::error::ErrorWrapperComponent,
            >>::Delegate,
    }
}
