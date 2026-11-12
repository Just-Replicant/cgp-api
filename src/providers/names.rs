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
/// `#[cgp_impl]` must be in scope at the call site (`use cgp_api::prelude::*`).
#[macro_export]
macro_rules! entity_name {
    ($provider:ident, $name:literal) => {
        /// Entity-name provider.
        pub struct $provider;

        impl $crate::providers::names::EntityNameValue for $provider {
            const NAME: &'static str = $name;
        }

        #[cgp_impl($provider)]
        impl $crate::accessors::EntityNameGetter {
            fn entity_name(&self) -> &str {
                $name
            }
        }
    };
}
