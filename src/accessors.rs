//! Application accessors that do not belong in `cgp-orm`.
//!
//! [`HasEntityName`] is a routing or display name for a context
//! (`"posts"`, `"users"`, …). The name is an application policy, not a
//! repository concern.

use cgp_fork::prelude::*;

/// REST / display name for this context's entity.
#[cgp_getter]
pub trait HasEntityName {
    /// A stable, URL-safe entity name chosen by the application.
    fn entity_name(&self) -> &str;
}
