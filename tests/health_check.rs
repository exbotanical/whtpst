use gossip::{
    config::{get_config, DatabaseSettings},
    dao::{InMemoryRepository, Repository},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".into();
    let subscriber_name = "test".into();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();

    let mut config = get_config().expect("Failed to read config");
    config.database.database_name = Uuid::new_v4().to_string();
    let repo = InMemoryRepository::new();
    let db_pool = configure_db(&config.database).await;

    let server = run(listener, db_pool.clone(), repo).expect("Failed to bind addr");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

pub async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.sans_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create test database");

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate test database");

    pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn paste_returns_a_200_and_paste_id_when_ok() {
    let app = spawn_app().await;
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
async fn get_paste_returns_paste_when_exists() {
    let app = spawn_app().await;
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let paste_id = "abc";

    let response = client
        .get(&format!("{}/paste/{}", &app.address, &paste_id))
        .send()
        .await
        .expect("Failed to execute request");
    // assert_eq!(404, response.status().as_u16());

    // let payload = response.text().await.expect("Failed to get response data");
    // assert_eq!(paste_id, payload);
}

#[tokio::test]
async fn paste_returns_a_400_when_invalid_paste_id() {
    let app = spawn_app().await;
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

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();

//     // TODO: const
//     let body = "name=le%20villecoux&email=al_le_villecoux%40test.com";

//     let response = client
//         .post(&format!("{}/subscribe", &app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request");

//     assert_eq!(200, response.status().as_u16());

//     let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
//         .fetch_one(&app.db_pool)
//         .await
//         .expect("Failed to fetch saved subscription");

//     assert_eq!(saved.email, "al_le_villecoux@test.com");
//     assert_eq!(saved.name, "le villecoux");
// }

// #[tokio::test]
// async fn subscribe_returns_a_400_when_data_is_missing() {
//     let app = spawn_app().await;

//     let client = reqwest::Client::new();

//     let test_cases = vec![
//         ("name=le%20villecoux", "missing email"),
//         ("email=al_le_villecoux%40gmail.com", "missing name"),
//         ("", "missing both name and email"),
//     ];

//     for (invalid_body, error_message) in test_cases {
//         let response = client
//             .post(&format!("{}/subscribe", &app.address))
//             .header("Content-Type", "application/x-www-form-urlencoded")
//             .body(invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute request");

//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             "The API did not fail with 400 Bad Request when the payload was {}",
//             error_message
//         );
//     }
// }

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
