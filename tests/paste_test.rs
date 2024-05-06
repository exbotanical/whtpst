mod utils;

// GET /paste
#[tokio::test]
async fn get_paste_returns_paste_when_exists() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = "abc";
    let paste_content = "somecontent";

    let response = client
        .post(&format!("{}/paste/{}", &app.address, &paste_id))
        .header("Content-Type", "text/plain")
        .body(paste_content)
        .send()
        .await
        .expect("Failed to execute request");

    let paste_id = response.text().await.expect("Failed to get response data");

    let response = client
        .get(&format!("{}/paste/{}", &app.address, &paste_id))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(paste_content, payload);
}

#[tokio::test]
async fn get_paste_returns_404_when_not_found() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = "abc";

    let response = client
        .get(&format!("{}/paste/{}", &app.address, &paste_id))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(404, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(format!("Not found: {}", paste_id), payload);
}

#[tokio::test]
async fn get_paste_returns_400_when_bad_id() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = ":{";

    let response = client
        .get(&format!("{}/paste/{}", &app.address, &paste_id))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(400, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(
        format!("{} is not a valid paste id - invalid char", paste_id),
        payload
    );
}

// POST /paste

#[tokio::test]
async fn paste_returns_200_and_id_when_ok() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = "abc";

    let response = client
        .post(&format!("{}/paste/{}", &app.address, &paste_id))
        .header("Content-Type", "text/plain")
        .body("somecontent")
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(paste_id, payload);
}

#[tokio::test]
async fn paste_returns_400_when_invalid_content() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = "abc";

    let response = client
        .post(&format!("{}/paste/{}", &app.address, &paste_id))
        .header("Content-Type", "text/plain")
        .body("")
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(400, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!("not valid paste content - empty string", payload);
}

#[tokio::test]
async fn paste_returns_400_when_invalid_id() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = "{s";

    let response = client
        .post(&format!("{}/paste/{}", &app.address, &paste_id))
        .header("Content-Type", "text/plain")
        .body("somecontent")
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(400, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(
        format!("{} is not a valid paste id - invalid char", paste_id),
        payload
    );
}

#[tokio::test]
async fn paste_generates_id_when_none_provided() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/paste", &app.address))
        .header("Content-Type", "text/plain")
        .body("somecontent")
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(36, payload.len()); // UUID
}

// Full workflows
#[tokio::test]
async fn generated_id_retrieves_paste() {
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();

    let content = "somecontent";

    let response = client
        .post(&format!("{}/paste", &app.address))
        .header("Content-Type", "text/plain")
        .body(content)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");

    let response = client
        .get(&format!("{}/paste/{}", &app.address, payload))
        .header("Content-Type", "text/plain")
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());

    let payload = response.text().await.expect("Failed to get response data");
    assert_eq!(content, payload);
}
