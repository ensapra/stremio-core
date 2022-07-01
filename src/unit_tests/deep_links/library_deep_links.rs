use crate::deep_links::LibraryDeepLinks;
use crate::models::library_with_filters::{LibraryRequest, Sort};
use std::convert::TryFrom;

#[test]
fn library_deep_links_string() {
    let root = "library".to_string();
    let ldl = LibraryDeepLinks::try_from(&root).unwrap();
    assert_eq!(ldl.library, "stremio:///library".to_string());
}

#[test]
fn library_deep_links_request_type() {
    let root = "library".to_string();
    let request = LibraryRequest {
        r#type: Some("movie".to_string()),
        sort: Sort::LastWatched,
        page: Default::default(),
    };
    let ldl = LibraryDeepLinks::try_from((&root, &request)).unwrap();
    assert_eq!(
        ldl.library,
        "stremio:///library/movie?sort=lastwatched&page=1".to_string()
    );
}

#[test]
fn library_deep_links_request_no_type() {
    let root = "library".to_string();
    let request = LibraryRequest {
        r#type: None,
        sort: Sort::LastWatched,
        page: Default::default(),
    };
    let ldl = LibraryDeepLinks::try_from((&root, &request)).unwrap();
    assert_eq!(
        ldl.library,
        "stremio:///library?sort=lastwatched&page=1".to_string()
    );
}
