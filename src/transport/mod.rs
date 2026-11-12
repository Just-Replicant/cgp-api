//! Transport-independent response helpers.
//!
//! These wrappers are generic over the payload and optional metadata.
//! They are not tied to protobuf, HTTP, or a particular DTO. Transports
//! own the actual HTTP / RPC mapping; callers invoke resource-service
//! traits on a context and pass the result through
//! [`response_from`] / [`response_list_from`] / [`response_from_page`].

use cgp_orm::pagination::Page;

/// A single-item response with optional metadata and an error list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceResponse<T, M = ()> {
    /// The payload, if the operation produced one.
    pub data: Option<T>,
    /// Optional transport or pagination metadata.
    pub meta: Option<M>,
    /// Human-readable errors (empty on success).
    pub errors: Vec<String>,
}

impl<T, M> ResourceResponse<T, M> {
    /// Wrap `data` with empty metadata and errors.
    #[must_use]
    pub fn from_data(data: T) -> Self {
        Self {
            data: Some(data),
            meta: None,
            errors: Vec::new(),
        }
    }
}

/// A list response, optionally carrying page metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceResponseList<T, M = ()> {
    /// One response per item, in the repository's deterministic order.
    pub data: Vec<ResourceResponse<T, M>>,
    /// Optional list-level metadata (for example [`PageMeta`]).
    pub meta: Option<M>,
    /// Human-readable errors (empty on success).
    pub errors: Vec<String>,
}

/// Pagination metadata taken from a [`Page`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMeta {
    /// 1-based page that was fetched (after policy normalization).
    pub page: u64,
    /// Page size that was applied (after clamping).
    pub size: u64,
    /// Total number of matching rows.
    pub total: u64,
}

/// Wrap a single payload in a [`ResourceResponse`] with empty metadata.
#[must_use]
pub fn response_from<T>(data: T) -> ResourceResponse<T> {
    ResourceResponse::from_data(data)
}

/// Wrap a list of payloads in a [`ResourceResponseList`].
#[must_use]
pub fn response_list_from<T>(items: impl IntoIterator<Item = T>) -> ResourceResponseList<T> {
    ResourceResponseList {
        data: items.into_iter().map(ResourceResponse::from_data).collect(),
        meta: None,
        errors: Vec::new(),
    }
}

/// Wrap a [`Page`] as a list response, copying pagination into [`PageMeta`].
#[must_use]
pub fn response_from_page<T>(page: Page<T>) -> ResourceResponseList<T, PageMeta> {
    let meta = PageMeta {
        page: page.page,
        size: page.size,
        total: page.total,
    };
    ResourceResponseList {
        data: page
            .items
            .into_iter()
            .map(|item| ResourceResponse {
                data: Some(item),
                meta: Some(meta),
                errors: Vec::new(),
            })
            .collect(),
        meta: Some(meta),
        errors: Vec::new(),
    }
}
