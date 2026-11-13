//! Attribute grammar for `#[entity_api]`.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Ident, LitStr, Path, Result, Token, Type, parenthesized};

/// Parsed `#[entity_api(…)]` arguments, after defaults and cross-field checks.
pub struct EntityApiArgs {
    /// The primary context's stack (shorthand or explicit paths).
    pub stack: StackSpec,
    /// Context error type (from a keyword policy, or the required `error =`).
    pub error: TokenStream,
    /// `#[cfg(feature = "…")]` to emit, if any.
    pub cfg: Option<LitStr>,
    pub entity: Type,
    pub unique: Type,
    pub name: Option<Type>,
    pub resource: Option<ResourceArgs>,
    pub raise: Vec<Type>,
    pub wrap: Vec<Type>,
    pub extra: Vec<ComponentMapping>,
    pub also: Vec<AlsoEntry>,
}

/// How a context picks its preset parents.
pub enum StackSpec {
    /// `policy` + `layer` → ORM stack(s) + error policy.
    Shorthand { policy: Policy, layer: Layer },
    /// `stack = A` or `stack = A + B + C`, emitted as written.
    Explicit(Vec<Path>),
}

/// An error-policy preset: a keyword or a custom path.
pub enum Policy {
    Api,
    Rest,
    Grpc,
    /// Custom error-policy preset. `error` is required. `cfg` is never implied.
    Path(Path),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Lookup,
    Crud,
    Resource,
}

pub struct ResourceArgs {
    pub create: Type,
    pub update: Type,
    pub output: Type,
    pub codec: Codec,
}

pub struct SplitCodec {
    pub decode: Type,
    pub apply: Type,
    pub encode: Type,
}

pub enum Codec {
    Unified(Box<Type>),
    Split(Box<SplitCodec>),
}

pub struct ComponentMapping {
    pub component: Type,
    pub provider: Type,
}

pub struct AlsoEntry {
    pub name: Ident,
    pub stack: StackSpec,
    pub error: TokenStream,
    pub cfg: Option<LitStr>,
}

impl Policy {
    pub fn error_type(&self) -> Option<TokenStream> {
        Some(match self {
            Self::Api => quote::quote!(::cgp_api::error::ApiError),
            Self::Rest => quote::quote!(::cgp_api::error::RestError),
            // Re-export, so a consumer does not need a direct `tonic` dependency.
            Self::Grpc => quote::quote!(::cgp_api::error::Status),
            Self::Path(_) => return None,
        })
    }

    pub fn preset_path(&self) -> TokenStream {
        match self {
            Self::Api => quote::quote!(::cgp_api::presets::ApiErrorPolicy),
            Self::Rest => quote::quote!(::cgp_api::presets::RestErrorPolicy),
            Self::Grpc => quote::quote!(::cgp_api::presets::GrpcErrorPolicy),
            Self::Path(path) => quote::quote!(#path),
        }
    }

    pub fn is_keyword(&self) -> bool {
        !matches!(self, Self::Path(_))
    }
}

impl Layer {
    pub fn from_ident(ident: &Ident) -> Result<Self> {
        match ident.to_string().as_str() {
            "lookup" => Ok(Self::Lookup),
            "crud" => Ok(Self::Crud),
            "resource" => Ok(Self::Resource),
            other => Err(Error::new(
                ident.span(),
                format!("unknown layer `{other}`; expected `lookup`, `crud`, or `resource`"),
            )),
        }
    }

    pub fn is_resource(self) -> bool {
        matches!(self, Self::Resource)
    }
}

/// Parse `api` / `rest` / `grpc` as a keyword policy; any other path is custom.
pub fn policy_from_path(path: Path) -> Policy {
    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments[0].arguments.is_empty()
    {
        match path.segments[0].ident.to_string().as_str() {
            "api" => return Policy::Api,
            "rest" => return Policy::Rest,
            "grpc" => return Policy::Grpc,
            _ => {}
        }
    }
    Policy::Path(path)
}

impl Parse for ComponentMapping {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let component: Type = input.parse()?;
        input.parse::<Token![:]>()?;
        let provider: Type = input.parse()?;
        Ok(Self {
            component,
            provider,
        })
    }
}

/// Intermediate bag of optional keys, filled left-to-right.
struct RawArgs {
    policy: Option<(Policy, Span)>,
    layer: Option<(Layer, Span)>,
    stack: Option<(Vec<Path>, Span)>,
    error: Option<(Type, Span)>,
    cfg: Option<LitStr>,
    entity: Option<Type>,
    unique: Option<Type>,
    name: Option<Type>,
    create: Option<(Type, Span)>,
    update: Option<(Type, Span)>,
    output: Option<(Type, Span)>,
    codec: Option<(Type, Span)>,
    decode: Option<(Type, Span)>,
    apply: Option<(Type, Span)>,
    encode: Option<(Type, Span)>,
    raise: Vec<Type>,
    wrap: Vec<Type>,
    extra: Vec<ComponentMapping>,
    also: Vec<RawAlso>,
}

struct RawAlso {
    name: Ident,
    kind: RawAlsoKind,
    error: Option<(Type, Span)>,
    cfg: Option<LitStr>,
    layer: Option<(Layer, Span)>,
}

enum RawAlsoKind {
    Policy(Policy),
    Stack(Vec<Path>),
}

impl Parse for EntityApiArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let raw = parse_raw(input)?;
        finish(raw)
    }
}

fn parse_raw(input: ParseStream<'_>) -> Result<RawArgs> {
    let mut raw = RawArgs {
        policy: None,
        layer: None,
        stack: None,
        error: None,
        cfg: None,
        entity: None,
        unique: None,
        name: None,
        create: None,
        update: None,
        output: None,
        codec: None,
        decode: None,
        apply: None,
        encode: None,
        raise: Vec::new(),
        wrap: Vec::new(),
        extra: Vec::new(),
        also: Vec::new(),
    };

    while !input.is_empty() {
        let key: Ident = input.parse()?;
        let key_span = key.span();
        match key.to_string().as_str() {
            "policy" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.policy, key_span, "policy")?;
                let path: Path = input.parse()?;
                raw.policy = Some((policy_from_path(path), key_span));
            }
            "layer" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.layer, key_span, "layer")?;
                let ident: Ident = input.parse()?;
                raw.layer = Some((Layer::from_ident(&ident)?, ident.span()));
            }
            "stack" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.stack, key_span, "stack")?;
                let paths = parse_path_sum(input)?;
                raw.stack = Some((paths, key_span));
            }
            "error" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.error, key_span, "error")?;
                let ty: Type = input.parse()?;
                raw.error = Some((ty, key_span));
            }
            "cfg" => {
                input.parse::<Token![=]>()?;
                if raw.cfg.is_some() {
                    return Err(Error::new(key_span, "duplicate `cfg`"));
                }
                raw.cfg = Some(input.parse()?);
            }
            "entity" => {
                input.parse::<Token![=]>()?;
                if raw.entity.is_some() {
                    return Err(Error::new(key_span, "duplicate `entity`"));
                }
                raw.entity = Some(input.parse()?);
            }
            "unique" => {
                input.parse::<Token![=]>()?;
                if raw.unique.is_some() {
                    return Err(Error::new(key_span, "duplicate `unique`"));
                }
                raw.unique = Some(input.parse()?);
            }
            "name" => {
                input.parse::<Token![=]>()?;
                if raw.name.is_some() {
                    return Err(Error::new(key_span, "duplicate `name`"));
                }
                raw.name = Some(input.parse()?);
            }
            "create" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.create, key_span, "create")?;
                let ty: Type = input.parse()?;
                raw.create = Some((ty, key_span));
            }
            "update" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.update, key_span, "update")?;
                let ty: Type = input.parse()?;
                raw.update = Some((ty, key_span));
            }
            "output" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.output, key_span, "output")?;
                let ty: Type = input.parse()?;
                raw.output = Some((ty, key_span));
            }
            "codec" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.codec, key_span, "codec")?;
                let ty: Type = input.parse()?;
                raw.codec = Some((ty, key_span));
            }
            "decode" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.decode, key_span, "decode")?;
                let ty: Type = input.parse()?;
                raw.decode = Some((ty, key_span));
            }
            "apply" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.apply, key_span, "apply")?;
                let ty: Type = input.parse()?;
                raw.apply = Some((ty, key_span));
            }
            "encode" => {
                input.parse::<Token![=]>()?;
                reject_dup(&raw.encode, key_span, "encode")?;
                let ty: Type = input.parse()?;
                raw.encode = Some((ty, key_span));
            }
            "raise" => {
                if !raw.raise.is_empty() {
                    return Err(Error::new(key_span, "duplicate `raise`"));
                }
                raw.raise = parse_type_list(input)?;
            }
            "wrap" => {
                if !raw.wrap.is_empty() {
                    return Err(Error::new(key_span, "duplicate `wrap`"));
                }
                raw.wrap = parse_type_list(input)?;
            }
            "extra" => {
                if !raw.extra.is_empty() {
                    return Err(Error::new(key_span, "duplicate `extra`"));
                }
                let content;
                parenthesized!(content in input);
                raw.extra = Punctuated::<ComponentMapping, Token![,]>::parse_terminated(&content)?
                    .into_iter()
                    .collect();
            }
            "also" => {
                if !raw.also.is_empty() {
                    return Err(Error::new(key_span, "duplicate `also`"));
                }
                let content;
                parenthesized!(content in input);
                raw.also = parse_also_list(&content)?;
            }
            other => {
                return Err(Error::new(
                    key_span,
                    format!(
                        "unknown `entity_api` argument `{other}`; expected \
                         `policy`, `layer`, `stack`, `error`, `cfg`, `entity`, \
                         `create`, `update`, `output`, `codec`, `decode`, `apply`, \
                         `encode`, `unique`, `name`, `raise`, `wrap`, `extra`, or `also`"
                    ),
                ));
            }
        }

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
    }

    Ok(raw)
}

fn reject_dup<T>(existing: &Option<T>, span: Span, name: &str) -> Result<()> {
    if existing.is_some() {
        Err(Error::new(span, format!("duplicate `{name}`")))
    } else {
        Ok(())
    }
}

fn parse_path_sum(input: ParseStream<'_>) -> Result<Vec<Path>> {
    let paths = Punctuated::<Path, Token![+]>::parse_separated_nonempty(input)?;
    Ok(paths.into_iter().collect())
}

fn parse_type_list(input: ParseStream<'_>) -> Result<Vec<Type>> {
    let content;
    parenthesized!(content in input);
    let list = Punctuated::<Type, Token![,]>::parse_terminated(&content)?;
    Ok(list.into_iter().collect())
}

fn parse_also_list(input: ParseStream<'_>) -> Result<Vec<RawAlso>> {
    let mut entries = Vec::new();
    while !input.is_empty() {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        entries.push(parse_also_entry(name, input)?);
        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
    }
    Ok(entries)
}

fn parse_also_entry(name: Ident, input: ParseStream<'_>) -> Result<RawAlso> {
    // `Name: policy = Path`
    if peek_ident_eq(input, "policy") {
        let _policy: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let path: Path = input.parse()?;
        let trailing = parse_also_trailing(input)?;
        return Ok(RawAlso {
            name,
            kind: RawAlsoKind::Policy(policy_from_path(path)),
            error: trailing.error,
            cfg: trailing.cfg,
            layer: trailing.layer,
        });
    }

    let first: Path = input.parse()?;
    let policy = policy_from_path(first.clone());
    if policy.is_keyword() && !input.peek(Token![+]) {
        let trailing = parse_also_trailing(input)?;
        return Ok(RawAlso {
            name,
            kind: RawAlsoKind::Policy(policy),
            error: trailing.error,
            cfg: trailing.cfg,
            layer: trailing.layer,
        });
    }

    let mut paths = vec![first];
    while input.peek(Token![+]) {
        input.parse::<Token![+]>()?;
        paths.push(input.parse()?);
    }
    let trailing = parse_also_trailing(input)?;
    Ok(RawAlso {
        name,
        kind: RawAlsoKind::Stack(paths),
        error: trailing.error,
        cfg: trailing.cfg,
        layer: trailing.layer,
    })
}

/// Trailing `error`, `cfg`, and `layer` on an `also` entry. Stops before the
/// next `Name:` entry (ident followed by `:`).
struct AlsoTrailing {
    error: Option<(Type, Span)>,
    cfg: Option<LitStr>,
    layer: Option<(Layer, Span)>,
}

fn parse_also_trailing(input: ParseStream<'_>) -> Result<AlsoTrailing> {
    let mut error = None;
    let mut cfg = None;
    let mut layer = None;

    loop {
        if !input.peek(Token![,]) {
            break;
        }
        let fork = input.fork();
        let _: Token![,] = fork.parse()?;
        if fork.is_empty() {
            // Trailing comma at the end of `also(…)` or before a close.
            input.parse::<Token![,]>()?;
            break;
        }
        if fork.peek(Ident) {
            let ident: Ident = fork.parse()?;
            if fork.peek(Token![:]) && !fork.peek(Token![::]) && !fork.peek(Token![=]) {
                // Next also entry starts with `Name:`.
                break;
            }
            if ident == "error" || ident == "cfg" || ident == "layer" {
                if !fork.peek(Token![=]) {
                    return Err(Error::new(
                        ident.span(),
                        format!("expected `=` after `{ident}`"),
                    ));
                }
                input.parse::<Token![,]>()?;
                let key: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                match key.to_string().as_str() {
                    "error" => {
                        reject_dup(&error, key.span(), "error")?;
                        let ty: Type = input.parse()?;
                        error = Some((ty, key.span()));
                    }
                    "cfg" => {
                        if cfg.is_some() {
                            return Err(Error::new(key.span(), "duplicate `cfg`"));
                        }
                        cfg = Some(input.parse()?);
                    }
                    "layer" => {
                        reject_dup(&layer, key.span(), "layer")?;
                        let ident: Ident = input.parse()?;
                        layer = Some((Layer::from_ident(&ident)?, ident.span()));
                    }
                    other => {
                        return Err(Error::new(
                            key.span(),
                            format!(
                                "unknown `also` argument `{other}`; expected `error`, `cfg`, or `layer`"
                            ),
                        ));
                    }
                }
                continue;
            }
        }
        break;
    }

    Ok(AlsoTrailing { error, cfg, layer })
}

fn peek_ident_eq(input: ParseStream<'_>, name: &str) -> bool {
    let fork = input.fork();
    matches!(fork.parse::<Ident>(), Ok(ident) if ident == name && fork.peek(Token![=]))
}

fn finish(raw: RawArgs) -> Result<EntityApiArgs> {
    let mut errors = ErrorList::new();

    if raw.entity.is_none() {
        errors.push(Error::new(Span::call_site(), "missing `entity = …`"));
    }
    if raw.unique.is_none() {
        errors.push(Error::new(Span::call_site(), "missing `unique = …`"));
    }

    if raw.policy.is_some() && raw.stack.is_some() {
        let span = raw
            .stack
            .as_ref()
            .map(|(_, s)| *s)
            .unwrap_or_else(Span::call_site);
        errors.push(Error::new(
            span,
            "`policy` and `stack` cannot be combined; use `policy` + `layer` for a \
             shorthand, or `stack` for an explicit preset path (or `+`-sum)",
        ));
    }
    if raw.stack.is_some() && raw.layer.is_some() {
        let span = raw
            .layer
            .as_ref()
            .map(|(_, s)| *s)
            .unwrap_or_else(Span::call_site);
        errors.push(Error::new(
            span,
            "`layer` cannot be combined with `stack`; the stack paths already choose \
             the ORM parents",
        ));
    }

    let (stack, error_tokens, cfg, parent_layer) = match (&raw.policy, &raw.stack) {
        (Some((policy, policy_span)), None) => {
            let layer = raw.layer.map(|(l, _)| l).unwrap_or(Layer::Resource);
            finish_policy(
                policy,
                *policy_span,
                layer,
                raw.error.as_ref(),
                raw.cfg.clone(),
                &mut errors,
            )
        }
        (None, Some((paths, stack_span))) => {
            if paths.is_empty() {
                errors.push(Error::new(
                    *stack_span,
                    "`stack` must name at least one preset",
                ));
            }
            let error = match &raw.error {
                Some((ty, _)) => ty.to_token_stream(),
                None => {
                    errors.push(Error::new(
                        *stack_span,
                        "`error` is required when `stack` is explicit",
                    ));
                    TokenStream::new()
                }
            };
            (
                StackSpec::Explicit(paths.clone()),
                error,
                raw.cfg.clone(),
                None,
            )
        }
        (None, None) => {
            errors.push(Error::new(
                Span::call_site(),
                "either `policy = api | rest | grpc | {path}` or `stack = …` is required",
            ));
            (
                StackSpec::Explicit(Vec::new()),
                TokenStream::new(),
                raw.cfg.clone(),
                None,
            )
        }
        (Some(_), Some(_)) => {
            // Already reported the combination error. Still try to finish.
            (
                StackSpec::Explicit(Vec::new()),
                TokenStream::new(),
                raw.cfg.clone(),
                None,
            )
        }
    };

    let resource_layer = match &stack {
        StackSpec::Shorthand { layer, .. } => Some(*layer),
        StackSpec::Explicit(_) => {
            // Explicit stacks accept resource fields; if any is present, all are required.
            let any_resource = raw.create.is_some()
                || raw.update.is_some()
                || raw.output.is_some()
                || raw.codec.is_some()
                || raw.decode.is_some()
                || raw.apply.is_some()
                || raw.encode.is_some();
            if any_resource {
                Some(Layer::Resource)
            } else {
                Some(Layer::Lookup) // treat as non-resource (reject nothing extra)
            }
        }
    };

    let resource = finish_resource(resource_layer, &raw, &mut errors);

    let also = finish_also(raw.also, parent_layer, resource.is_some(), &mut errors);

    errors.fail()?;

    Ok(EntityApiArgs {
        stack,
        error: error_tokens,
        cfg,
        entity: raw.entity.expect("reported"),
        unique: raw.unique.expect("reported"),
        name: raw.name,
        resource,
        raise: raw.raise,
        wrap: raw.wrap,
        extra: raw.extra,
        also,
    })
}

fn finish_policy(
    policy: &Policy,
    policy_span: Span,
    layer: Layer,
    error: Option<&(Type, Span)>,
    cfg: Option<LitStr>,
    errors: &mut ErrorList,
) -> (StackSpec, TokenStream, Option<LitStr>, Option<Layer>) {
    let error_tokens = match (policy.error_type(), error) {
        (Some(implied), None) => implied,
        (Some(_), Some((_, span))) => {
            errors.push(Error::new(
                *span,
                "`error` is implied by a keyword `policy` (`api`, `rest`, or `grpc`); \
                 remove `error`, or use a custom policy path",
            ));
            TokenStream::new()
        }
        (None, Some((ty, _))) => ty.to_token_stream(),
        (None, None) => {
            errors.push(Error::new(
                policy_span,
                "`error` is required when `policy` is a custom preset path",
            ));
            TokenStream::new()
        }
    };
    // `cfg` stays as written. A keyword policy does not add
    // `#[cfg(feature = "thiserror")]` (or `rest` / `grpc`): application
    // crates usually enable those only on the `cgp-api` dependency.
    (
        StackSpec::Shorthand {
            policy: match policy {
                Policy::Api => Policy::Api,
                Policy::Rest => Policy::Rest,
                Policy::Grpc => Policy::Grpc,
                Policy::Path(p) => Policy::Path(p.clone()),
            },
            layer,
        },
        error_tokens,
        cfg,
        Some(layer),
    )
}

fn finish_resource(
    layer: Option<Layer>,
    raw: &RawArgs,
    errors: &mut ErrorList,
) -> Option<ResourceArgs> {
    let layer = layer?;

    let resource_keys = [
        raw.create.as_ref().map(|(_, s)| ("create", *s)),
        raw.update.as_ref().map(|(_, s)| ("update", *s)),
        raw.output.as_ref().map(|(_, s)| ("output", *s)),
        raw.codec.as_ref().map(|(_, s)| ("codec", *s)),
        raw.decode.as_ref().map(|(_, s)| ("decode", *s)),
        raw.apply.as_ref().map(|(_, s)| ("apply", *s)),
        raw.encode.as_ref().map(|(_, s)| ("encode", *s)),
    ];

    if !layer.is_resource() {
        for (name, span) in resource_keys.into_iter().flatten() {
            errors.push(Error::new(
                span,
                format!(
                    "`{name}` is only valid with `layer = resource` (lookup and crud \
                     reject resource-service arguments)"
                ),
            ));
        }
        return None;
    }

    if raw.codec.is_some() && (raw.decode.is_some() || raw.apply.is_some() || raw.encode.is_some())
    {
        let span = raw
            .decode
            .as_ref()
            .or(raw.apply.as_ref())
            .or(raw.encode.as_ref())
            .map(|(_, s)| *s)
            .unwrap_or_else(Span::call_site);
        errors.push(Error::new(
            span,
            "`codec` cannot be combined with `decode` / `apply` / `encode`; \
             use one codec for all three, or name them separately",
        ));
    }

    let codec = if let Some((ty, _)) = &raw.codec {
        Some(Codec::Unified(Box::new(ty.clone())))
    } else if raw.decode.is_some() || raw.apply.is_some() || raw.encode.is_some() {
        match (&raw.decode, &raw.apply, &raw.encode) {
            (Some((d, _)), Some((a, _)), Some((e, _))) => {
                Some(Codec::Split(Box::new(SplitCodec {
                    decode: d.clone(),
                    apply: a.clone(),
                    encode: e.clone(),
                })))
            }
            _ => {
                let span = raw
                    .decode
                    .as_ref()
                    .or(raw.apply.as_ref())
                    .or(raw.encode.as_ref())
                    .map(|(_, s)| *s)
                    .unwrap_or_else(Span::call_site);
                errors.push(Error::new(
                    span,
                    "`decode`, `apply`, and `encode` must all be set when not using `codec`",
                ));
                None
            }
        }
    } else {
        None
    };

    let missing: Vec<&str> = [
        raw.create.is_none().then_some("create"),
        raw.update.is_none().then_some("update"),
        raw.output.is_none().then_some("output"),
        codec
            .is_none()
            .then_some("codec (or decode / apply / encode)"),
    ]
    .into_iter()
    .flatten()
    .collect();

    if !missing.is_empty() {
        errors.push(Error::new(
            Span::call_site(),
            format!(
                "`layer = resource` requires {}, and a codec (`codec = …` or \
                 `decode` / `apply` / `encode`); missing: {}",
                "`create`, `update`, `output`",
                missing.join(", ")
            ),
        ));
        return None;
    }

    Some(ResourceArgs {
        create: raw.create.as_ref().unwrap().0.clone(),
        update: raw.update.as_ref().unwrap().0.clone(),
        output: raw.output.as_ref().unwrap().0.clone(),
        codec: codec.unwrap(),
    })
}

fn finish_also(
    raw: Vec<RawAlso>,
    parent_layer: Option<Layer>,
    parent_has_resource: bool,
    errors: &mut ErrorList,
) -> Vec<AlsoEntry> {
    let mut entries = Vec::new();
    for also in raw {
        match also.kind {
            RawAlsoKind::Policy(policy) => {
                if policy.is_keyword()
                    && let Some((_, span)) = &also.error
                {
                    errors.push(Error::new(
                        *span,
                        "`error` is implied by a keyword policy on an `also` entry",
                    ));
                }

                let explicit_layer = also.layer.map(|(layer, _)| layer);
                let layer = explicit_layer.or(parent_layer);
                let layer = match layer {
                    Some(layer) => layer,
                    None if policy.is_keyword() => {
                        errors.push(Error::new(
                            also.name.span(),
                            format!(
                                "`also({}: {})` keeps the parent layer, but this annotation \
                                 used `stack` rather than `policy` and `layer`. Write an \
                                 explicit `stack` (or `policy = …` with `layer` and `error`) \
                                 for this `also` entry",
                                also.name,
                                also_policy_label(&policy),
                            ),
                        ));
                        continue;
                    }
                    None => {
                        errors.push(Error::new(
                            also.name.span(),
                            format!(
                                "`also({}: policy = …)` needs `layer` when the parent used \
                                 an explicit `stack`",
                                also.name
                            ),
                        ));
                        continue;
                    }
                };

                if explicit_layer.is_some_and(Layer::is_resource) && !parent_has_resource {
                    errors.push(Error::new(
                        also.name.span(),
                        format!(
                            "`also({}: …, layer = resource)` needs `create`, `update`, \
                             `output`, and a codec on this annotation; those fields live \
                             on the shared base, so the primary context has to be \
                             `layer = resource` or an explicit stack that sets them",
                            also.name
                        ),
                    ));
                    continue;
                }

                let error = match (policy.error_type(), &also.error) {
                    (Some(implied), None) => implied,
                    (None, Some((ty, _))) => ty.to_token_stream(),
                    (None, None) => {
                        errors.push(Error::new(
                            also.name.span(),
                            format!(
                                "`error` is required when `also({}: …)` uses a custom \
                                 policy path",
                                also.name
                            ),
                        ));
                        continue;
                    }
                    (Some(_), Some(_)) => continue, // already reported
                };

                entries.push(AlsoEntry {
                    name: also.name,
                    stack: StackSpec::Shorthand { policy, layer },
                    error,
                    cfg: also.cfg,
                });
            }
            RawAlsoKind::Stack(paths) => {
                if let Some((_, span)) = also.layer {
                    errors.push(Error::new(
                        span,
                        "`layer` cannot be combined with an explicit `also` stack",
                    ));
                }
                let error = match &also.error {
                    Some((ty, _)) => ty.to_token_stream(),
                    None => {
                        let span = paths
                            .first()
                            .map(Spanned::span)
                            .unwrap_or_else(|| also.name.span());
                        errors.push(Error::new(
                            span,
                            format!(
                                "`error` is required when `also({}: …)` uses an explicit stack",
                                also.name
                            ),
                        ));
                        continue;
                    }
                };
                entries.push(AlsoEntry {
                    name: also.name,
                    stack: StackSpec::Explicit(paths),
                    error,
                    cfg: also.cfg,
                });
            }
        }
    }
    entries
}

fn also_policy_label(policy: &Policy) -> String {
    match policy {
        Policy::Api => "api".into(),
        Policy::Rest => "rest".into(),
        Policy::Grpc => "grpc".into(),
        Policy::Path(p) => p.to_token_stream().to_string(),
    }
}

struct ErrorList(Vec<Error>);

impl ErrorList {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, error: Error) {
        self.0.push(error);
    }

    fn fail(self) -> Result<()> {
        let mut iter = self.0.into_iter();
        let Some(mut first) = iter.next() else {
            return Ok(());
        };
        for error in iter {
            first.combine(error);
        }
        Err(first)
    }
}
