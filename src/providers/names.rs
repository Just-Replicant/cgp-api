//! [`ConstEntityName`]: a [`HasEntityName`](crate::accessors::HasEntityName)
//! provider that returns a compile-time string.
//!
//! Rust does not (yet) allow `&'static str` as a const generic, so the
//! name is supplied by a zero-size tag type implementing [`EntityNameValue`]:
//!
//! ```ignore
//! struct Posts;
//! impl EntityNameValue for Posts { const NAME: &'static str = "posts"; }
//! // UniqueColumnsGetterComponent-style wiring:
//! // EntityNameGetterComponent: ConstEntityName<Posts>
//! ```
//!
//! The [`entity_name`](crate::entity_name) macro expands to the same
//! pattern.

use cgp_fork::prelude::*;

use crate::accessors::{EntityNameGetter, EntityNameGetterComponent};

/// A compile-time entity name.
pub trait EntityNameValue {
    /// The name, for example `"posts"`.
    const NAME: &'static str;
}

/// Provider that returns [`EntityNameValue::NAME`].
pub struct ConstEntityName<Name>(core::marker::PhantomData<Name>);

#[cgp_impl(ConstEntityName<Name>)]
impl<Name: EntityNameValue> EntityNameGetter {
    fn entity_name(&self) -> &str {
        Name::NAME
    }
}

/// Declare a [`crate::accessors::HasEntityName`] provider with a fixed name.
///
/// ```ignore
/// cgp_api::entity_name!(PostEntityName, "posts");
/// ```
///
/// The generated type is public and implements the getter through CGP.
/// Expansion mirrors `#[cgp_impl]` with fully qualified paths, so the call
/// site needs neither the prelude nor `EntityNameGetterComponent` in scope
/// (`cgp_impl` would emit that name bare, which fails under macro hygiene).
///
/// The name is a compile-time `&'static str` literal. The tag trait
/// [`EntityNameValue`] is kept until `adt_const_params` allows `&str` as
/// a const generic.
#[macro_export]
macro_rules! entity_name {
    ($provider:ident, $name:literal) => {
        /// Entity-name provider.
        pub struct $provider;

        impl $crate::providers::names::EntityNameValue for $provider {
            const NAME: &'static str = $name;
        }

        impl<__Context__> $crate::accessors::EntityNameGetter<__Context__> for $provider {
            fn entity_name(__context__: &__Context__) -> &str {
                $name
            }
        }

        impl<__Context__>
            ::cgp_fork::macro_prelude::IsProviderFor<
                $crate::accessors::EntityNameGetterComponent,
                __Context__,
                (),
            > for $provider
        {
        }
    };
}

#[cfg(test)]
mod tests {
    // Deliberately no `use cgp_fork::prelude::*`, no `cgp_impl`, and no
    // `EntityNameGetterComponent` import: the macro must expand with
    // fully qualified paths alone.
    crate::entity_name!(Posts, "posts");

    #[test]
    fn entity_name_macro_does_not_need_prelude() {
        assert_eq!(<Posts as crate::providers::EntityNameValue>::NAME, "posts");
        assert_eq!(
            <Posts as crate::accessors::EntityNameGetter<()>>::entity_name(&()),
            "posts"
        );
    }
}
