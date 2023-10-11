/*

use mockito;
use tokio::time::Duration;
use xkcd_rs::{download_comic, get_latest_comic};

#[tokio::test]
async fn integration_test_get_latest_comic() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let mock = server.mock("GET", "/info.0.json")
        .with_status(200)
        .with_body(r#"{ "num": 2400, "title": "Test", "img": "https://example.com/test.png", "alt": "Test comic" }"#)
        .expect(1)
        .create();

    let result = get_latest_comic(url, 3, Duration::from_secs(1)).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2400);
    mock.assert();
}

#[tokio::test]
async fn integration_test_download_comic() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let mock = server.mock("GET", "/2400/info.0.json")
        .with_status(200)
        .with_body(r#"{ "num": 2400, "title": "Test", "img": "https://example.com/test.png", "alt": "Test comic" }"#)
        .expect(1)
        .create();

    let result = download_comic(url, 2400, 3, Duration::from_secs(1)).await;

    assert!(result.is_ok());
    mock.assert();
}
*/
