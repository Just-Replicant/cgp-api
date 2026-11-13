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
//! The [`entity_name`](crate::entity_name) macro expands to the tag and
//! the [`EntityNameValue`] impl. Wire [`ConstEntityName`]`<Tag>` as the
//! [`EntityNameGetter`] provider.

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

/// Declare a tag type that implements [`EntityNameValue`] with a fixed name.
///
/// ```ignore
/// cgp_api::entity_name!(Posts, "posts");
/// // EntityNameGetterComponent: ConstEntityName<Posts>
/// ```
///
/// The generated type is a public tag. It does **not** implement
/// [`crate::accessors::EntityNameGetter`]; that impl lives once on
/// [`ConstEntityName`]. Wire `ConstEntityName<Tag>` as the
/// [`crate::accessors::EntityNameGetterComponent`] provider.
///
/// The name is a compile-time `&'static str` literal. The tag trait
/// [`EntityNameValue`] is kept until `adt_const_params` allows `&str` as
/// a const generic.
#[macro_export]
macro_rules! entity_name {
    ($provider:ident, $name:literal) => {
        /// Entity-name tag for [`$crate::providers::ConstEntityName`].
        pub struct $provider;

        impl $crate::providers::names::EntityNameValue for $provider {
            const NAME: &'static str = $name;
        }
    };
}

#[cfg(test)]
mod tests {
    // Deliberately no `use cgp_fork::prelude::*` and no `cgp_impl` import:
    // the macro only declares a tag and an `EntityNameValue` impl.
    crate::entity_name!(Posts, "posts");

    #[test]
    fn entity_name_macro_does_not_need_prelude() {
        assert_eq!(<Posts as crate::providers::EntityNameValue>::NAME, "posts");
    }

    #[test]
    fn entity_name_macro_is_used_through_const_entity_name() {
        use cgp_fork::prelude::*;

        use crate::accessors::HasEntityName;
        use crate::providers::ConstEntityName;

        #[derive(HasField)]
        struct Ctx;

        delegate_components! {
            #[unchecked]
            Ctx {
                crate::accessors::EntityNameGetterComponent: ConstEntityName<Posts>,
            }
        }

        assert_eq!(Ctx.entity_name(), "posts");
    }
}
