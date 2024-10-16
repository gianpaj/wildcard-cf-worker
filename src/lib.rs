use serde::{Deserialize, Serialize};
use wildcard::Wildcard;
use worker::*;

#[derive(Deserialize, Serialize, Debug)]
struct Json {
    message: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct JsonCapture {
    captures: Vec<String>,
}

#[event(fetch)]
async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let queries = _req.uri().path_and_query().and_then(|pq| pq.query());

    let url = queries
        .and_then(|q| q.split('&').find(|p| p.starts_with("url=")))
        .and_then(|p| p.split('=').nth(1));

    let wildcard = queries
        .and_then(|q| q.split('&').find(|p| p.starts_with("wildcard=")))
        .and_then(|p| p.split('=').nth(1));

    if let (Some(url), Some(wildcard)) = (url, wildcard) {
        console_log!("url: {}", url);
        console_log!("wildcard: {}", wildcard);

        let wildcard_o = Wildcard::new(wildcard.as_bytes()).unwrap();

        if let Some(captures) = wildcard_o.captures(url.as_bytes()) {
            for (i, capture) in captures.iter().enumerate() {
                console_log!("Capture {}: {}", i, String::from_utf8_lossy(capture));
            }

            Response::from_json(&JsonCapture {
                captures: captures
                    .iter()
                    .map(|c| String::from_utf8_lossy(c).to_string())
                    .collect::<Vec<String>>(),
            })
        } else {
            console_log!("No captures found");
            Response::from_json(&Json {
                message: String::from("No captures found"),
            })
        }
    } else {
        console_log!("No url or wildcard found");
        Response::from_json(&Json {
            message: String::from("No url or wildcard found"),
        })
    }
}
