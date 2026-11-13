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
/// The `#[cgp_impl]` attribute is invoked at the absolute path
/// `::cgp_fork::prelude::cgp_impl`, so the call site does not need the
/// prelude (or any other `cgp_impl` import) in scope.
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

        // The getter component must be in scope for `#[cgp_impl]`. An
        // anonymous `const` block keeps that `use` from colliding when the
        // macro is invoked more than once in the same module.
        const _: () = {
            use $crate::accessors::EntityNameGetterComponent;
            #[::cgp_fork::prelude::cgp_impl($provider)]
            impl $crate::accessors::EntityNameGetter {
                fn entity_name(&self) -> &str {
                    $name
                }
            }
        };
    };
}

#[cfg(test)]
mod tests {
    // Deliberately no `use cgp_fork::prelude::*` and no `cgp_impl` import:
    // the macro must expand at an absolute path.
    crate::entity_name!(Posts, "posts");

    #[test]
    fn entity_name_macro_does_not_need_prelude() {
        assert_eq!(<Posts as crate::providers::EntityNameValue>::NAME, "posts");
    }
}
