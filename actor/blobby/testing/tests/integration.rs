use reqwest::{Response, StatusCode};

mod setup;

#[tokio::test]
async fn test_happy_path() {
    let (url, _guard) = setup::setup_test().await;

    let client = reqwest::Client::new();

    let res = client
        .post(format!("http://{}/prequel.txt", url))
        .body("This works Anakin?")
        .send()
        .await
        .expect("Should be able to do http request");

    assert_response(res, StatusCode::OK, None).await;

    // Now make sure put works as well (testing with a container this time)
    let res = client
        .put(format!("http://{}/sequel.txt?container=foobar", url))
        .body("I totally can pass a lightsaber over a distance magically")
        .send()
        .await
        .expect("Should be able to do http request");

    assert_response(res, StatusCode::OK, None).await;

    // And try a container via header as well
    let res = client
        .put(format!("http://{}/sequel.txt", url))
        .header("blobby-container", "header")
        .body("Episode 7 is _definitely_ different from 4")
        .send()
        .await
        .expect("Should be able to do http request");

    assert_response(res, StatusCode::OK, None).await;

    // Now try fetching all of the files we created
    let res = client
        .get(format!("http://{}/prequel.txt", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::OK, Some("This works Anakin?")).await;

    let res = client
        .get(format!("http://{}/sequel.txt?container=foobar", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(
        res,
        StatusCode::OK,
        Some("I totally can pass a lightsaber over a distance magically"),
    )
    .await;

    let res = client
        .get(format!("http://{}/sequel.txt", url))
        .header("blobby-container", "header")
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(
        res,
        StatusCode::OK,
        Some("Episode 7 is _definitely_ different from 4"),
    )
    .await;

    // Now write over some data
    let res = client
        .post(format!("http://{}/prequel.txt", url))
        .body("Right?")
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::OK, None).await;

    // Fetch it again to make sure it updated
    let res = client
        .get(format!("http://{}/prequel.txt", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::OK, Some("Right?")).await;

    // Now delete and make sure it is gone
    let res = client
        .delete(format!("http://{}/prequel.txt", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::OK, None).await;

    let res = client
        .delete(format!("http://{}/sequel.txt", url))
        .header("blobby-container", "header")
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::OK, None).await;

    let res = client
        .get(format!("http://{}/prequel.txt", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::NOT_FOUND, None).await;

    let res = client
        .get(format!("http://{}/sequel.txt", url))
        .header("blobby-container", "header")
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::NOT_FOUND, None).await;
}

#[tokio::test]
async fn test_bad_requests() {
    let (url, _guard) = setup::setup_test().await;
    let client = reqwest::Client::new();
    // Test for missing file
    let res = client
        .get(format!("http://{}/totallydoesntexist", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_response(res, StatusCode::NOT_FOUND, None).await;

    // Test unsupported header
    let res = client
        .head(format!("http://{}/totallydoesntexist", url))
        .send()
        .await
        .expect("Should be able to do http request");
    assert_eq!(
        res.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "Should have returned method not allowed"
    );
    assert_eq!(
        res.headers()
            .get(reqwest::header::ALLOW)
            .expect("Allow header should be set"),
        "GET, POST, PUT, DELETE",
        "Allowed methods not correct"
    );
}

// Helper that asserts the response has the given status and equals the optional body
async fn assert_response(res: Response, expected: StatusCode, expected_body: Option<&str>) {
    assert_eq!(
        res.status(),
        expected,
        "Expected a {} response. Got: {}",
        expected,
        res.status()
    );

    if let Some(eb) = expected_body {
        let raw = res.bytes().await.expect("Unable to read body");
        let body = String::from_utf8_lossy(raw.as_ref());
        assert_eq!(
            body, eb,
            "Body did got match. Got: {}, Expected: {}",
            body, expected
        )
    }
}
