use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

use anyhow::Result;
use s3::bucket::Bucket;
use s3::creds::Credentials;

#[derive(Deserialize)]
struct Request {
    contest_name: String,
    seed: usize,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let _contest_name = event.payload.contest_name;
    let _seed = event.payload.seed;

    // s3
    // https://docs.rs/rust-s3/latest/s3/bucket/struct.Bucket.html#method.get_object
    let bucket_name = "procon-inputs";
    let region = "ap-northeast-1".parse()?;
    let credentials = Credentials::default()?;
    let bucket = Bucket::new(bucket_name, region, credentials)?;
    let response_data = bucket.get_object("/ahc012/0019.txt").await?;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("response_data {:?}", response_data),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
