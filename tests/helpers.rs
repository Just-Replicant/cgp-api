//! Unit tests for the generic transport helpers (and REST conversion).

use cgp_api::prelude::*;
use cgp_orm::pagination::Page;

#[test]
fn response_from_wraps_payload() {
    let response = response_from("hello");
    assert_eq!(response.data, Some("hello"));
    assert_eq!(response.meta, None);
    assert!(response.errors.is_empty());
}

#[test]
fn response_list_from_preserves_order() {
    let list = response_list_from(["a", "b", "c"]);
    let items: Vec<_> = list
        .data
        .iter()
        .map(|item| *item.data.as_ref().unwrap())
        .collect();
    assert_eq!(items, ["a", "b", "c"]);
    assert!(list.meta.is_none());
    assert!(list.errors.is_empty());
}

#[test]
fn response_from_page_copies_pagination() {
    let page = Page {
        items: vec![1, 2],
        page: 2,
        size: 10,
        total: 12,
    };
    let list = response_from_page(page);
    let meta = list.meta.expect("page meta");
    assert_eq!(
        meta,
        PageMeta {
            page: 2,
            size: 10,
            total: 12
        }
    );
    assert_eq!(list.data.len(), 2);
    assert_eq!(list.data[0].meta, Some(meta));
    assert_eq!(list.data[0].data, Some(1));
}

#[cfg(feature = "rest")]
#[test]
fn rest_error_converts_to_axum_pair() {
    use axum::http::StatusCode;
    use cgp_api::error::RestError;

    let error = RestError {
        status: StatusCode::NOT_FOUND,
        message: "missing".into(),
    };
    let (status, message): (StatusCode, String) = error.into();
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(message, "missing");
}
