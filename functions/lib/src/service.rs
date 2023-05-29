use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client;

async fn config() -> SdkConfig {
    aws_config::load_from_env().await
}

pub async fn ddb() -> Client {
    let sc = config().await;
    Client::new(&sc)
}
