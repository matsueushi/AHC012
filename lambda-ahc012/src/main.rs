use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};
use std::io::BufReader;

use anyhow::Result;
use s3::bucket::Bucket;
use s3::creds::Credentials;

use proconio::source::line::LineSource;

use solver::*;

#[derive(Deserialize)]
struct Request {
    bucket_in: String,
    bucket_out: String,
    contest_name: String,
    seed: usize,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    path_in: String,
    path_out: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let bucket_in = event.payload.bucket_in;
    let bucket_out = event.payload.bucket_out;
    let contest_name = event.payload.contest_name;
    let seed = event.payload.seed;

    // s3 input
    // https://docs.rs/rust-s3/latest/s3/bucket/struct.Bucket.html#method.get_object
    let region = "ap-northeast-1".parse()?;
    let credentials = Credentials::default()?;

    let bucket_in = Bucket::new(&bucket_in, region, credentials)?;
    let path_in = format!("{}/{:04}.txt", contest_name, seed);
    let response_data = bucket_in.get_object(&path_in).await?;

    let buf_reader = BufReader::new(response_data.as_slice());
    let mut source = LineSource::new(buf_reader);

    // parse input
    let input = Input::from_source(&mut source);

    // s3 output
    let region = "ap-northeast-1".parse()?;
    let credentials = Credentials::default()?;
    let bucket_out = Bucket::new(&bucket_out, region, credentials)?;
    let path_out = format!("{}/{:04}.json", contest_name, seed);

    // 結果を出力
    let content = "{}".as_bytes();
    bucket_out.put_object(&path_out, content).await?;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        path_in,
        path_out,
        msg: format!("input {:?}", input),
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
