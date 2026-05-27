use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::Value;
use tower::util::ServiceExt;
use zyblog::routes::videos::video_routes;

/// Build the videos router for testing.
fn videos_app() -> Router {
    video_routes()
}

#[tokio::test]
async fn upload_video_missing_file_field() {
    let app = videos_app();

    let boundary = "----TestBoundary123";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"notfile\"\r\n\r\n\
         some data\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/videos")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn upload_video_unsupported_type() {
    let app = videos_app();

    let boundary = "----TestBoundary123";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\
         Content-Type: text/plain\r\n\r\n\
         not a video\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/videos")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn upload_video_rejects_image_type() {
    let app = videos_app();

    let boundary = "----TestBoundary123";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.jpg\"\r\n\
         Content-Type: image/jpeg\r\n\r\n\
         not a video\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/videos")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn upload_video_valid_mp4() {
    let app = videos_app();

    let boundary = "----TestBoundary123";
    // Minimal fake MP4 data
    let mp4_data: &[u8] = &[0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70];

    let mut body = Vec::new();
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        b"Content-Disposition: form-data; name=\"file\"; filename=\"test.mp4\"\r\n",
    );
    body.extend_from_slice(b"Content-Type: video/mp4\r\n\r\n");
    body.extend_from_slice(mp4_data);
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/videos")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(json.get("url").is_some());
    assert!(json.get("filename").is_some());
    let url = json["url"].as_str().unwrap();
    assert!(url.starts_with("/static/videos/"));
    assert!(url.ends_with(".mp4"));
}

#[tokio::test]
async fn upload_video_valid_webm() {
    let app = videos_app();

    let boundary = "----TestBoundary123";
    let webm_data: &[u8] = &[0x1A, 0x45, 0xDF, 0xA3];

    let mut body = Vec::new();
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        b"Content-Disposition: form-data; name=\"file\"; filename=\"test.webm\"\r\n",
    );
    body.extend_from_slice(b"Content-Type: video/webm\r\n\r\n");
    body.extend_from_slice(webm_data);
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/videos")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();
    let url = json["url"].as_str().unwrap();
    assert!(url.ends_with(".webm"));
}

#[tokio::test]
async fn upload_video_error_response_is_json() {
    let app = videos_app();

    let boundary = "----TestBoundary123";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"notfile\"\r\n\r\n\
         some data\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/videos")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(json.get("error").is_some());
}
