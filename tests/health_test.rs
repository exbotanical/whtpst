mod utils;

#[tokio::test]
async fn health_check_works() {
    let app = utils::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// #[tokio::test]
// async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();

//     let test_cases = vec![
//         ("name=&email=someone_name%40gmail.com", "empty name"),
//         ("name=Someone&email=", "empty email"),
//         (
//             "name=Someone&email=definitely-not-an-email",
//             "invalid email",
//         ),
//     ];

//     for (body, description) in test_cases {
//         let response = client
//             .post(&format!("{}/subscribe", &app.address))
//             .header("Content-Type", "application/x-www-form-urlencoded")
//             .body(body)
//             .send()
//             .await
//             .expect("Failed to execute request.");

//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             "The API did not return a 400 Bad Request when the payload was {}.",
//             description
//         );
//     }
// }
