use std::{env, error::Error, sync::Arc};

use env_logger::Target;
use log::LevelFilter;
use tmhi::nokia;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        .init();
    log::info!("Hello, world!");

    let username = env::var("TMHI_USERNAME").expect("TMHI_USERNAME env var is not set");
    let password = env::var("TMHI_PASSWORD").expect("TMHI_PASSWORD env var is not set");

    let client = Arc::new(nokia::Client::new(&username, &password));
    let client2 = Arc::clone(&client);

    // todo: should not be logging in twice
    let (two, one) = tokio::join!(
        tokio::spawn(async move {
            client.login().await.unwrap();
        }),
        tokio::spawn(async move {
            client2.login().await.unwrap();
        })
    );
    two.unwrap();
    one.unwrap();

    Ok(())
}
