//! Unit tests for the generic transport helpers (and REST conversion).

use cgp_api::prelude::*;
use cgp_orm::pagination::Page;

#[test]
fn response_list_from_moves_the_vec_in() {
    let items = vec!["a", "b", "c"];
    let buffer = items.as_ptr();
    let list = response_list_from(items);
    assert_eq!(list.data, ["a", "b", "c"]);
    assert_eq!(list.data.as_ptr(), buffer, "the Vec must not be rebuilt");
    assert!(list.meta.is_none());
    assert_eq!(list.len(), 3);
}

#[test]
fn response_from_page_moves_items_and_puts_meta_on_the_list_only() {
    let page = Page {
        items: vec![1, 2],
        page: 2,
        size: 10,
        total: Some(12),
    };
    let expected_meta = PageMeta::of(&page);
    let buffer = page.items.as_ptr();

    let list = response_from_page(page);
    assert_eq!(list.data.as_ptr(), buffer, "page items must be moved in");
    assert_eq!(list.data, [1, 2]);
    assert_eq!(
        list.meta,
        Some(PageMeta {
            page: 2,
            size: 10,
            total: Some(12)
        })
    );
    assert_eq!(list.meta, Some(expected_meta));
}

#[test]
fn page_without_total_keeps_total_none() {
    let page: Page<u8> = Page {
        items: Vec::new(),
        page: 1,
        size: 20,
        total: None,
    };
    let list: ResourceResponseList<u8, PageMeta> = page.into();
    assert!(list.is_empty());
    assert_eq!(list.meta.map(|meta| meta.total), Some(None));
}

#[test]
fn list_iterates_payloads_and_splits_into_parts() {
    let list = ResourceResponseList::with_meta(vec![1, 2, 3], "meta");
    let borrowed: Vec<_> = (&list).into_iter().copied().collect();
    assert_eq!(borrowed, [1, 2, 3]);
    let (data, meta) = list.clone().into_parts();
    assert_eq!(data, [1, 2, 3]);
    assert_eq!(meta, Some("meta"));
    let owned: Vec<_> = list.into_iter().collect();
    assert_eq!(owned, [1, 2, 3]);
}

#[test]
fn empty_list_does_not_allocate() {
    let list: ResourceResponseList<String> = ResourceResponseList::default();
    assert_eq!(list.data.capacity(), 0);
    assert!(list.meta.is_none());
}

#[cfg(feature = "rest")]
#[test]
fn rest_error_converts_to_axum_pair() {
    use axum::http::StatusCode;
    use cgp_api::error::RestError;

    let error = RestError::new(StatusCode::NOT_FOUND, "missing");
    assert_eq!(error.to_string(), "missing");
    let (status, message): (StatusCode, String) = error.into();
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(message, "missing");
}
