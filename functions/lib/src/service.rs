use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_lambda::Client as LambdaClient;

async fn config() -> SdkConfig {
    aws_config::load_from_env().await
}

pub async fn make_dynamo_client() -> DynamoClient {
    DynamoClient::new(&config().await)
}

pub async fn make_lambda_client() -> LambdaClient {
    LambdaClient::new(&config().await)
}
