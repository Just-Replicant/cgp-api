//! Presets, context struct, `new`, and `delegate_components!`.

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Fields, Ident, ItemStruct, Result, Token, Visibility};

use crate::check::expand_checks;
use crate::parse::{
    Codec, ComponentMapping, EntityApiArgs, Layer, ResourceArgs, SplitCodec, StackSpec,
};

pub fn expand(args: EntityApiArgs, item: ItemStruct) -> Result<TokenStream> {
    validate_struct(&item)?;
    validate_also_names(&args, &item.ident)?;

    let fields = named_fields(&item)?;
    let field_names: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().expect("named field"))
        .collect();
    let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let field_vis: Vec<_> = fields.iter().map(|field| &field.vis).collect();

    let base = format_ident!("{}Base", item.ident);
    let mut tokens = expand_base(&base, &args);
    let attrs = forwarded_attrs(&item.attrs)?;

    let fields = ContextFields {
        names: &field_names,
        types: &field_types,
        vis: &field_vis,
    };

    tokens.extend(expand_one(ContextEmit {
        name: &item.ident,
        vis: &item.vis,
        attrs,
        fields,
        base: &base,
        args: &args,
        stack: &args.stack,
        error: &args.error,
        cfg: cfg_attr(&args.cfg),
    }));

    for also in &args.also {
        tokens.extend(expand_one(ContextEmit {
            name: &also.name,
            vis: &item.vis,
            attrs: TokenStream::new(),
            fields,
            base: &base,
            args: &args,
            stack: &also.stack,
            error: &also.error,
            cfg: cfg_attr(&also.cfg),
        }));
    }

    Ok(tokens)
}

fn validate_struct(item: &ItemStruct) -> Result<()> {
    let mut errors = Vec::new();

    if !item.generics.params.is_empty() {
        errors.push(Error::new(
            item.generics.span(),
            "`#[entity_api]` does not support generic context structs",
        ));
    }

    match &item.fields {
        Fields::Named(fields) => {
            let mut saw_db_conn = false;
            for field in &fields.named {
                if let Err(error) = field.modifiers.require_empty() {
                    errors.push(error);
                }
                if let Some((_, default)) = &field.default {
                    errors.push(Error::new_spanned(
                        default,
                        "`#[entity_api]` does not support field default values",
                    ));
                }
                if field.ident.as_ref().is_some_and(|name| name == "db_conn") {
                    saw_db_conn = true;
                }
            }
            if !saw_db_conn {
                errors.push(Error::new_spanned(
                    item,
                    "`#[entity_api]` requires a `db_conn` field; cgp-orm derives `HasDbConn` from that field name",
                ));
            }
        }
        Fields::Unnamed(_) => {
            errors.push(Error::new_spanned(
                item,
                "`#[entity_api]` requires a named-field struct, not a tuple struct",
            ));
        }
        Fields::Unit => {
            errors.push(Error::new_spanned(
                item,
                "`#[entity_api]` requires a named-field struct, not a unit struct",
            ));
        }
    }

    combine(errors)
}

fn validate_also_names(args: &EntityApiArgs, struct_name: &Ident) -> Result<()> {
    let mut errors = Vec::new();
    for (index, also) in args.also.iter().enumerate() {
        if also.name == *struct_name {
            errors.push(Error::new_spanned(
                &also.name,
                "`also(…)` name must differ from the annotated struct",
            ));
        }
        if args.also[..index]
            .iter()
            .any(|other| other.name == also.name)
        {
            errors.push(Error::new_spanned(
                &also.name,
                "duplicate `also(…)` context name",
            ));
        }
    }
    combine(errors)
}

fn named_fields(
    item: &ItemStruct,
) -> Result<&syn::punctuated::Punctuated<syn::Field, syn::Token![,]>> {
    match &item.fields {
        Fields::Named(fields) => Ok(&fields.named),
        _ => Err(Error::new_spanned(
            item,
            "`#[entity_api]` requires a named-field struct",
        )),
    }
}

fn combine(errors: Vec<Error>) -> Result<()> {
    let mut iter = errors.into_iter();
    let Some(mut first) = iter.next() else {
        return Ok(());
    };
    for error in iter {
        first.combine(error);
    }
    Err(first)
}

/// Keep the struct's attributes, dropping `Clone` and `HasField` from
/// `#[derive(...)]` because the expansion always adds those.
fn forwarded_attrs(attrs: &[syn::Attribute]) -> Result<TokenStream> {
    let mut out = TokenStream::new();
    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let Some(derive) = strip_added_derives(attr)? {
                out.extend(derive);
            }
        } else {
            out.extend(attr.to_token_stream());
        }
    }
    Ok(out)
}

fn strip_added_derives(attr: &syn::Attribute) -> Result<Option<TokenStream>> {
    let paths: Punctuated<syn::Path, Token![,]> =
        attr.parse_args_with(Punctuated::parse_terminated)?;
    let kept: Vec<_> = paths
        .into_iter()
        .filter(|path| !is_added_derive(path))
        .collect();
    if kept.is_empty() {
        Ok(None)
    } else {
        Ok(Some(quote!(#[derive(#(#kept),*)])))
    }
}

fn is_added_derive(path: &syn::Path) -> bool {
    let Some(segment) = path.segments.last() else {
        return false;
    };
    (segment.ident == "Clone" || segment.ident == "HasField")
        && path
            .segments
            .iter()
            .all(|segment| segment.arguments.is_empty())
}

fn cfg_attr(cfg: &Option<syn::LitStr>) -> TokenStream {
    match cfg {
        Some(feature) => quote!(#[cfg(feature = #feature)]),
        None => quote!(),
    }
}

fn expand_base(base: &Ident, args: &EntityApiArgs) -> TokenStream {
    let entity = &args.entity;
    let unique = &args.unique;

    let resource_entries = args.resource.as_ref().map(resource_entries);

    let name_entry = args.name.as_ref().map(|tag| {
        quote! {
            ::cgp_api::accessors::EntityNameGetterComponent:
                ::cgp_api::providers::ConstEntityName<#tag>,
        }
    });

    quote! {
        ::cgp_fork::prelude::cgp_preset! {
            #base {
                ::cgp_orm::types::EntityTypeProviderComponent:
                    ::cgp_fork::prelude::UseType<#entity>,
                #resource_entries
                ::cgp_orm::accessors::UniqueColumnsGetterComponent: #unique,
                #name_entry
            }
        }
    }
}

fn resource_entries(resource: &ResourceArgs) -> TokenStream {
    let ResourceArgs {
        create,
        update,
        output,
        codec,
    } = resource;

    let codec_entries = match codec {
        Codec::Unified(codec) => quote! {
            [
                ::cgp_orm::components::DecodeCreateInputComponent,
                ::cgp_orm::components::ApplyUpdateInputComponent,
                ::cgp_orm::components::EncodeOutputComponent,
            ]: #codec,
        },
        Codec::Split(split) => {
            let SplitCodec {
                decode,
                apply,
                encode,
            } = &**split;
            quote! {
                ::cgp_orm::components::DecodeCreateInputComponent: #decode,
                ::cgp_orm::components::ApplyUpdateInputComponent: #apply,
                ::cgp_orm::components::EncodeOutputComponent: #encode,
            }
        }
    };

    quote! {
        ::cgp_orm::components::service::CreateInputTypeProviderComponent:
            ::cgp_fork::prelude::UseType<#create>,
        ::cgp_orm::components::service::UpdateInputTypeProviderComponent:
            ::cgp_fork::prelude::UseType<#update>,
        ::cgp_orm::components::service::OutputTypeProviderComponent:
            ::cgp_fork::prelude::UseType<#output>,
        #codec_entries
    }
}

/// Parent list for `{Name}Preset: {parents} + {Base}`.
fn parent_presets(stack: &StackSpec) -> TokenStream {
    match stack {
        StackSpec::Shorthand { policy, layer } => {
            let orm = match layer {
                Layer::Lookup => quote!(::cgp_orm::presets::RepositoryLookupStack),
                Layer::Crud => quote!(::cgp_orm::presets::RepositoryCrudStack),
                Layer::Resource => quote! {
                    ::cgp_orm::presets::RepositoryCrudStack
                    + ::cgp_orm::presets::ResourceServiceStack
                },
            };
            let policy = policy.preset_path();
            quote!(#orm + #policy)
        }
        StackSpec::Explicit(paths) => {
            let mut tokens = TokenStream::new();
            for (index, path) in paths.iter().enumerate() {
                if index > 0 {
                    tokens.extend(quote!(+));
                }
                tokens.extend(quote!(#path));
            }
            tokens
        }
    }
}

#[derive(Clone, Copy)]
struct ContextFields<'a> {
    names: &'a [&'a Ident],
    types: &'a [&'a syn::Type],
    vis: &'a [&'a Visibility],
}

struct ContextEmit<'a> {
    name: &'a Ident,
    vis: &'a Visibility,
    attrs: TokenStream,
    fields: ContextFields<'a>,
    base: &'a Ident,
    args: &'a EntityApiArgs,
    stack: &'a StackSpec,
    error: &'a TokenStream,
    cfg: TokenStream,
}

fn expand_one(emit: ContextEmit<'_>) -> TokenStream {
    let ContextEmit {
        name,
        vis,
        attrs,
        fields,
        base,
        args,
        stack,
        error,
        cfg,
    } = emit;
    let field_names = fields.names;
    let field_types = fields.types;
    let field_vis = fields.vis;
    let preset = format_ident!("{name}Preset");
    let parents = parent_presets(stack);
    let extra = args.extra.iter().map(
        |ComponentMapping {
             component,
             provider,
         }| { quote!(#component: #provider,) },
    );
    let checks = expand_checks(name, args, error, &cfg);

    quote! {
        #cfg
        ::cgp_fork::prelude::cgp_preset! {
            #preset: #parents + #base {}
        }

        #cfg
        #attrs
        #[derive(::cgp_fork::prelude::HasField, Clone)]
        #vis struct #name {
            #(#field_vis #field_names: #field_types,)*
        }

        #cfg
        impl #name {
            /// Build a context from its fields.
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self { #(#field_names),* }
            }
        }

        #cfg
        ::cgp_fork::prelude::delegate_components! {
            #[unchecked]
            #name {
                preset #preset,
                #(#extra)*
            }
        }

        #checks
    }
}
