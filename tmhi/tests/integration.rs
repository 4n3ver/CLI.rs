use std::env;

use tmhi::nokia::Client;

#[tokio::test]
async fn should_fetch_radio_status() {
    // given
    let client = Client::new("username", "password");

    // when
    let status = client.radio_status().await.unwrap();

    // then
    assert!(status.lte.first().unwrap().status.band.starts_with("B"));
    assert!(status.nr.first().unwrap().status.band.starts_with("n"));
}

#[tokio::test]
#[should_panic]
async fn should_fail_auth() {
    // given
    let client = Client::new("username", "password");

    // expect
    client.login().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn should_auth() {
    // given
    let username = env::var("TMHI_USERNAME").expect("TMHI_USERNAME env var is not set");
    let password = env::var("TMHI_PASSWORD").expect("TMHI_PASSWORD env var is not set");
    let client = Client::new(&username, &password);

    // expect
    client.login().await.unwrap();
}
