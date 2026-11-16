//! Transport-independent response helpers.
//!
//! A transport response is the payload, not a wrapper. A handler that
//! reads or writes one row returns that row's output type directly:
//! `get` is `Result<Option<Output>, Error>`, `create` is
//! `Result<(Output, InsertOutcome), Error>`, and a failure travels on the
//! `Err` side of the [`Result`], never in a second error channel next to
//! the payload. There is no single-item wrapper in this crate.
//!
//! The one shape added here is a list with **one** list-level metadata
//! value, [`ResourceResponseList`]. It owns a `Vec<T>` of payloads, not a
//! `Vec` of per-item wrappers, so building one from a [`Page`] or an
//! existing `Vec<T>` moves the buffer in and never reallocates.
//! [`PageMeta`] is [`Copy`] and sits on the list only. Per-item metadata
//! belongs to a caller that has a real per-item value (it makes `T` a
//! struct of its own), not to pagination.
//!
//! These types are generic over the payload and the metadata. They are not
//! tied to protobuf, HTTP, or a particular DTO. Transports own the actual
//! HTTP / RPC mapping; callers invoke resource-service traits on a context
//! and pass a list result through [`response_from_page`] /
//! [`response_list_from`] (or the equivalent [`From`] impls).

use cgp_orm::pagination::Page;

/// A list of payloads plus one optional list-level metadata value.
///
/// `data` holds the payloads themselves, in the repository's
/// deterministic order. `meta` is [`PageMeta`] for a paginated list and
/// `()` (left as `None`) otherwise.
///
/// Constructing a list never allocates: [`Self::new`] and
/// [`Self::with_meta`] take an owned `Vec<T>`, and the `From<Page<T>>`
/// impl (used by [`response_from_page`]) moves [`Page::items`] in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct ResourceResponseList<T, M = ()> {
    /// The payloads, in the repository's deterministic order.
    pub data: Vec<T>,
    /// Optional list-level metadata (for example [`PageMeta`]).
    pub meta: Option<M>,
}

impl<T, M> ResourceResponseList<T, M> {
    /// A list of `data` with no metadata. Moves the `Vec` in.
    pub const fn new(data: Vec<T>) -> Self {
        Self { data, meta: None }
    }

    /// A list of `data` with list-level `meta`. Moves the `Vec` in.
    pub const fn with_meta(data: Vec<T>, meta: M) -> Self {
        Self {
            data,
            meta: Some(meta),
        }
    }

    /// Number of payloads.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// `true` when the list holds no payloads.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Split into the payload `Vec` and the metadata, without copying.
    #[must_use]
    pub fn into_parts(self) -> (Vec<T>, Option<M>) {
        (self.data, self.meta)
    }
}

impl<T, M> Default for ResourceResponseList<T, M> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<T> From<Vec<T>> for ResourceResponseList<T> {
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}

impl<T> From<Page<T>> for ResourceResponseList<T, PageMeta> {
    fn from(page: Page<T>) -> Self {
        let meta = PageMeta::of(&page);
        Self::with_meta(page.items, meta)
    }
}

impl<T, M> IntoIterator for ResourceResponseList<T, M> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, M> IntoIterator for &'a ResourceResponseList<T, M> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

/// Pagination metadata taken from a [`Page`].
///
/// [`Copy`], and carried once per list, never per item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageMeta {
    /// 1-based page that was fetched (after policy normalization).
    pub page: u64,
    /// Page size that was applied (after clamping).
    pub size: u64,
    /// Total number of matching rows.
    ///
    /// `None` when the request skipped `COUNT(*)`
    /// ([`cgp_orm::pagination::PageRequest::without_total`]).
    pub total: Option<u64>,
}

impl PageMeta {
    /// Read the pagination fields of `page` without touching its items.
    #[must_use]
    pub const fn of<T>(page: &Page<T>) -> Self {
        Self {
            page: page.page,
            size: page.size,
            total: page.total,
        }
    }
}

impl<T> From<&Page<T>> for PageMeta {
    fn from(page: &Page<T>) -> Self {
        Self::of(page)
    }
}

/// Wrap an owned `Vec<T>` as a [`ResourceResponseList`] with no metadata.
///
/// The `Vec` is moved in, not copied or rebuilt.
pub fn response_list_from<T>(items: Vec<T>) -> ResourceResponseList<T> {
    ResourceResponseList::new(items)
}

/// Turn a [`Page`] into a list response with one [`PageMeta`].
///
/// [`Page::items`] is moved in (no reallocation), and the pagination
/// fields are copied once onto the list.
pub fn response_from_page<T>(page: Page<T>) -> ResourceResponseList<T, PageMeta> {
    ResourceResponseList::from(page)
}
