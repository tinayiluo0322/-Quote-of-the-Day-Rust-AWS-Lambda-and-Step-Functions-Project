use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

// data to push to the dynamodb table
#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuote {
    pub quote_id: String,
    pub quote_value: String,
}
// required for AWS
#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

// implement Display for the Failure response so that we can then implement Error.
impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    req_id: String,
    msg: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let req_id = event.payload.req_id;
    let msg = event.payload.msg;
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut quote = GetQuote {
        quote_id: String::new(),
        quote_value: String::new(),
    };
    // set value
    quote.quote_id = String::from(req_id.clone());
    quote.quote_value = String::from(msg);

    let quote_id = AttributeValue::S(quote.quote_id.clone());
    let quote_value = AttributeValue::S(quote.quote_value.to_string());
    // add to dynamodb
    // store our data in the quote table
    let _resp = client
        .put_item()
        .table_name("quote")
        .item("quote_id", quote_id)
        .item("quote_value", quote_value)
        .send()
        .await
        .map_err(|_err| FailureResponse {
            body: _err.to_string(),
        });
    println!("{:?}", _resp);
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: "Inserted into db".to_string(),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}