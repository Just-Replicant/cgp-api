//! Application accessors that do not belong in `cgp-orm`.
//!
//! [`HasEntityName`] is a routing or display name for a context
//! (`"posts"`, `"users"`, …). The name is an application policy, not a
//! repository concern.

use cgp_fork::prelude::*;

/// REST / display name for this context's entity.
///
/// The name is a compile-time `&'static str` literal, provided by
/// [`crate::providers::ConstEntityName`] (the [`crate::entity_name`]
/// macro produces a tag for that provider), not a runtime value loaded
/// from the database.
///
/// This is a regular component (provider trait [`EntityNameGetter`], key
/// `EntityNameGetterComponent`) rather than a `#[cgp_getter]` field
/// getter, because the name is a `'static` policy, not a borrow of a
/// context field. It follows the same shape as
/// [`cgp_orm::accessors::HasUniqueColumns`] in `cgp-orm` 0.1.3. Reading
/// it never allocates, and the returned `&'static str` is not tied to the
/// lifetime of the context.
#[cgp_component(EntityNameGetter)]
pub trait HasEntityName {
    /// A stable, URL-safe entity name chosen by the application.
    ///
    /// This is a compile-time literal.
    fn entity_name(&self) -> &'static str;
}
