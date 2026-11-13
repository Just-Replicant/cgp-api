//! `check_components!` and `{Name}Preset::with_components!` emission.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::parse::{ComponentMapping, EntityApiArgs};

/// Emit the raiser / wrapper checks and the inherited-key check for one context.
pub fn expand_checks(
    name: &Ident,
    args: &EntityApiArgs,
    error: &TokenStream,
    cfg: &TokenStream,
) -> TokenStream {
    let check_trait = format_ident!("CanUse{name}");
    let preset = format_ident!("{name}Preset");
    let entity = &args.entity;

    let raise = &args.raise;
    let wrap = &args.wrap;

    let extra_checks = args
        .extra
        .iter()
        .map(|ComponentMapping { component, .. }| quote!(#component,));
    let extra_exclude = args
        .extra
        .iter()
        .map(|ComponentMapping { component, .. }| quote!(#component,));

    let error_type = quote!(::cgp_fork::core::error::ErrorTypeProviderComponent);
    let raiser = quote!(::cgp_fork::core::error::ErrorRaiserComponent);
    let wrapper = quote!(::cgp_fork::core::error::ErrorWrapperComponent);

    quote! {
        #cfg
        ::cgp_fork::prelude::check_components! {
            #[check_trait(#check_trait)]
            #name {
                #(#extra_checks)*
                #error_type,
                #raiser: [
                    #error,
                    ::core::convert::Infallible,
                    ::sea_orm::DbErr,
                    String,
                    &'static str,
                    ::cgp_orm::error::ErrIdentityEmpty,
                    ::cgp_orm::error::ErrMissingPrimaryKey,
                    ::cgp_orm::error::ErrUnsupportedKeyArity,
                    ::cgp_orm::error::ErrMissingOrdering,
                    ::cgp_orm::error::ErrConflictFetchFailed,
                    ::cgp_orm::error::ErrEntityNotFound,
                    ::cgp_orm::error::ErrUniqueConflict,
                    ::cgp_orm::error::ErrIdentityFieldUnset<
                        <#entity as ::sea_orm::EntityTrait>::Column
                    >,
                    ::cgp_orm::error::ErrPrimaryKeyUnset<
                        <#entity as ::sea_orm::EntityTrait>::Column
                    >,
                    #(#raise,)*
                ],
                #wrapper: [
                    String,
                    &'static str,
                    #(#wrap,)*
                ],
            }
        }

        // Inherited keys except the three error components and any `extra`
        // keys. Those are checked with their real types on the context table
        // above. `extra` stays on `delegate_components!`, so a key that
        // already exists on a parent stack is replaced there.
        #cfg
        #preset::with_components! {
            @ ::cgp_fork::prelude::cgp_for_each,
            exclude [
                #error_type,
                #raiser,
                #wrapper,
                #(#extra_exclude)*
            ],
            __Component__ => {
                impl #check_trait<__Component__, ()> for #name {}
            }
        }
    }
}
