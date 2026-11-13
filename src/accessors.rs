//! Application accessors that do not belong in `cgp-orm`.
//!
//! [`HasEntityName`] is a routing or display name for a context
//! (`"posts"`, `"users"`, …). The name is an application policy, not a
//! repository concern.

use cgp_fork::prelude::*;

/// REST / display name for this context's entity.
///
/// The name is a compile-time `&'static str` literal, provided by
/// [`crate::providers::ConstEntityName`] or [`crate::entity_name`], not a
/// runtime value loaded from the database.
#[cgp_getter]
pub trait HasEntityName {
    /// A stable, URL-safe entity name chosen by the application.
    ///
    /// This is a compile-time literal.
    fn entity_name(&self) -> &str;
}
