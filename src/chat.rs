use eventsource_client as es;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub stream: Option<bool>,
}

pub async fn stream_response(client: impl es::Client) -> Message {
    let mut response = String::new();

    let mut stream = client.stream().map_ok(|event| match event {
        es::SSE::Event(ev) => {
            if ev.event_type == "message" {
                let body: serde_json::Value = serde_json::from_str(&ev.data).unwrap_or_default();
                let message = body["choices"][0]["delta"]["content"].as_str();

                if let Some(message) = message {
                    print!("{message}");
                    std::mem::drop(io::stdout().flush());
                    response.push_str(message);
                }
            }
        }
        es::SSE::Comment(_) => {}
    });

    let mut end = false;
    while !end {
        if let Err(err) = stream.try_next().await {
            if format!("{err:?}") != "Eof" {
                eprintln!("Error: {err:?}");
                break;
            }
            end = true;
        }
    }

    print!("\n\n");

    Message {
        role: "assistant".to_string(),
        content: response,
    }
}
