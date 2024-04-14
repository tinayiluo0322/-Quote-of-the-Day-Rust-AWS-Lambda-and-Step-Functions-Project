use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

use rand::Rng;
use serde::{Deserialize, Serialize};

// quote of the day generator
fn quoteoftheday(command: String) -> Result<String, Box<dyn std::error::Error>> {
    // lol
    print!("{command}");
    let result = if rand::thread_rng().gen::<bool>() {
        "The only way to do great work is to love what you do. - Steve Jobs"
    } else {
        "Success is not final, failure is not fatal: It is the courage to continue that counts. - Winston Churchill"
    };

    Ok(result.to_string())
}

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
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
    let command = event.payload.command;

    let message = match quoteoftheday(command.to_string()) {
        Ok(inference_result) => {
            format!("{}", inference_result)
        }
        Err(err) => {
            format!("Error in inference: {:?}", err)
        }
    };
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: message,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}