use zyblog::handlers::posts::{CreatePostRequest, ListPostsQuery, UpdatePostRequest};

// ===== CreatePostRequest deserialization tests =====

#[test]
fn create_post_request_deserialize_full() {
    let json = r#"{
        "title": "Test Post",
        "content": "Some content",
        "excerpt": "Short excerpt",
        "cover_image": "https://example.com/image.jpg",
        "status": "published"
    }"#;

    let req: CreatePostRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.title, "Test Post");
    assert_eq!(req.content, "Some content");
    assert_eq!(req.excerpt.as_deref(), Some("Short excerpt"));
    assert_eq!(req.cover_image.as_deref(), Some("https://example.com/image.jpg"));
    assert_eq!(req.status.as_deref(), Some("published"));
}

#[test]
fn create_post_request_deserialize_minimal() {
    let json = r#"{
        "title": "Test Post",
        "content": "Some content"
    }"#;

    let req: CreatePostRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.title, "Test Post");
    assert_eq!(req.content, "Some content");
    assert!(req.excerpt.is_none());
    assert!(req.cover_image.is_none());
    assert!(req.status.is_none());
}

// ===== UpdatePostRequest deserialization tests =====

#[test]
fn update_post_request_deserialize_partial() {
    let json = r#"{
        "title": "Updated Title"
    }"#;

    let req: UpdatePostRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.title.as_deref(), Some("Updated Title"));
    assert!(req.content.is_none());
    assert!(req.excerpt.is_none());
    assert!(req.cover_image.is_none());
    assert!(req.status.is_none());
}

#[test]
fn update_post_request_deserialize_all_fields() {
    let json = r#"{
        "title": "Updated",
        "content": "New content",
        "excerpt": "New excerpt",
        "cover_image": "https://example.com/new.jpg",
        "status": "draft"
    }"#;

    let req: UpdatePostRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.title.as_deref(), Some("Updated"));
    assert_eq!(req.content.as_deref(), Some("New content"));
    assert_eq!(req.excerpt.as_deref(), Some("New excerpt"));
    assert_eq!(req.cover_image.as_deref(), Some("https://example.com/new.jpg"));
    assert_eq!(req.status.as_deref(), Some("draft"));
}

// ===== ListPostsQuery deserialization tests =====

#[test]
fn list_posts_query_defaults() {
    let json = r#"{}"#;
    let query: ListPostsQuery = serde_json::from_str(json).unwrap();
    assert!(query.page.is_none());
    assert!(query.per_page.is_none());
    assert!(query.status.is_none());
}

#[test]
fn list_posts_query_with_values() {
    let json = r#"{
        "page": 2,
        "per_page": 20,
        "status": "published"
    }"#;
    let query: ListPostsQuery = serde_json::from_str(json).unwrap();
    assert_eq!(query.page, Some(2));
    assert_eq!(query.per_page, Some(20));
    assert_eq!(query.status.as_deref(), Some("published"));
}
