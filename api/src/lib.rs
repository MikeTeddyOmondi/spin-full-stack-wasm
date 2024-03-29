// use std::str::FromStr;

use anyhow::Result;
use http::request;
use serde_json::{json, Value};
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TodosRequest {
    pub data: String,
}

#[derive(Serialize)]
pub struct TodosResponse {
    pub data: Value,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> Result<Response> {
    // println!("{:#?}", req);
    // println!("{:#?}", req.headers());
    // Ok(http::Response::builder()
    //     .status(200)
    //     .header("foo", "bar")
    //     .body(Some("Hello, Fermyon".into()))?)

    let mut router = Router::new();
    router.get("/api/todos", get_todos);
    // router.post("/api/todos", post_todos);
    router.any("/api/*", not_found);
    router.handle(req)
}

fn not_found(_: Request, _: Params) -> Result<Response> {
    let json_value: Value = json!({
        "success": "false",
        "message": "Not found"
    });

    let json_string = json_value.to_string();
    println!("{}", json_string);

    Ok(http::Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(Some(json_string.into()))?)
}

fn get_todos(req: Request, _params: Params) -> Result<Response> {
    // println!("Request: {:#?}", req);

    // let request = body_json_to_map(&req)?;
    // println!("Bytes: {:#?}", request);

    let json_value: Value = json!({
        "success": "true",
        "message": "todos read"
    });

    // let json_string = json_value.to_string();
    // println!("{}", json_string);

    let resp = TodosResponse { data: json_value };

    let resp_str = serde_json::to_string(&resp)?;
    println!("{:#?}", resp_str);
    send_ok_response(200, resp_str)
}

fn send_ok_response(code: u16, resp_str: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(code)
        .header("Content-Type", "application/json")
        .body(Some(resp_str.into()))?)
}

fn body_json_to_map(req: &Request) -> Result<TodosRequest> {
    let body = match req.body().as_ref() {
        Some(bytes) => bytes,
        None => anyhow::bail!("Request body was unexpectedly empty"),
    };
    println!("Request body: {:#?}", body);

    Ok(serde_json::from_slice(&body)?)
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

// use std::str::FromStr;

// use anyhow::Result;
// use spin_sdk::{
//     http::{Params, Request, Response, Router},
//     http_component,
//     key_value::Store,
//     llm::{infer_with_options, InferencingModel::Llama2Chat},
// };

// use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// pub struct TodosRequest {
//     pub sentence: String,
// }

// #[derive(Serialize)]
// pub struct TodosResponse {
//     pub sentiment: String,
// }

// const PROMPT: &str = r#"\
// <<SYS>>
// You are a bot that generates sentiment analysis responses. Respond with a single positive, negative, or neutral.
// <</SYS>>
// <INST>
// Follow the pattern of the following examples:

// User: Hi, my name is Bob
// Bot: neutral

// User: I am so happy today
// Bot: positive

// User: I am so sad today
// Bot: negative
// </INST>

// User: {SENTENCE}
// "#;

// /// A Spin HTTP component that internally routes requests.
// #[http_component]
// fn handle_route(req: Request) -> Result<Response> {
//     let mut router = Router::new();
//     router.post("/api/sentiment-analysis", perform_sentiment_analysis);
//     router.any("/api/*", not_found);
//     router.handle(req)
// }

// fn not_found(_: Request, _: Params) -> Result<Response> {
//     Ok(http::Response::builder()
//         .status(404)
//         .body(Some("Not found".into()))?)
// }

// fn perform_sentiment_analysis(req: Request, _params: Params) -> Result<Response> {
//     let request = body_json_to_map(&req)?;
//     // Do some basic clean up on the input
//     let sentence = request.sentence.trim();
//     println!("Performing sentiment analysis on: {}", sentence);

//     // Prepare the KV store
//     let kv = Store::open_default()?;

//     // If the sentiment of the sentence is already in the KV store, return it
//     if kv.exists(sentence).unwrap_or(false) {
//         println!("Found sentence in KV store returning cached sentiment");
//         let sentiment = kv.get(sentence)?;
//         let resp = TodosResponse {
//             sentiment: String::from_utf8(sentiment)?,
//         };
//         let resp_str = serde_json::to_string(&resp)?;

//         return send_ok_response(200, resp_str)
//     }
//     println!("Sentence not found in KV store");

//     // Otherwise, perform sentiment analysis
//     println!("Running inference");
//     let inferencing_result = infer_with_options(
//         Llama2Chat,
//         &PROMPT.replace("{SENTENCE}", sentence),
//         spin_sdk::llm::InferencingParams {
//             max_tokens: 6,
//             ..Default::default()
//         },
//     )?;
//     println!("Inference result {:?}", inferencing_result);
//     let sentiment = inferencing_result
//         .text
//         .lines()
//         .next()
//         .unwrap_or_default()
//         .strip_prefix("Bot:")
//         .unwrap_or_default()
//         .parse::<Sentiment>();
//     println!("Got sentiment: {sentiment:?}");

//     if let Ok(sentiment) = sentiment {
//         println!("Caching sentiment in KV store");
//         let _ = kv.set(sentence, sentiment);
//     }
//     // Cache the result in the KV store
//     let resp = TodosResponse {
//         sentiment: sentiment
//             .as_ref()
//             .map(ToString::to_string)
//             .unwrap_or_default(),
//     };

//     let resp_str = serde_json::to_string(&resp)?;
//     send_ok_response(200, resp_str)
// }

// fn send_ok_response(code: u16, resp_str: String) -> Result<Response> {
//     Ok(http::Response::builder()
//     .status(code)
//     .body(Some(resp_str.into()))?)
// }

// fn body_json_to_map(req: &Request) -> Result<TodosRequest> {
//     let body = match req.body().as_ref() {
//         Some(bytes) => bytes,
//         None => anyhow::bail!("Request body was unexpectedly empty"),
//     };

//     Ok(serde_json::from_slice(&body)?)
// }

// #[derive(Copy, Clone, Debug)]
// enum Sentiment {
//     Positive,
//     Negative,
//     Neutral,
// }

// impl Sentiment {
//     fn as_str(&self) -> &str {
//         match self {
//             Self::Positive => "positive",
//             Self::Negative => "negative",
//             Self::Neutral => "neutral",
//         }
//     }
// }

// impl std::fmt::Display for Sentiment {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.as_str())
//     }
// }

// impl AsRef<[u8]> for Sentiment {
//     fn as_ref(&self) -> &[u8] {
//         self.as_str().as_bytes()
//     }
// }

// impl FromStr for Sentiment {
//     type Err = String;

//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         let sentiment = match s.trim() {
//             "positive" => Self::Positive,
//             "negative" => Self::Negative,
//             "neutral" => Self::Neutral,
//             _ => return Err(s.into()),
//         };
//         Ok(sentiment)
//     }
// }
