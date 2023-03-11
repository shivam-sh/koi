use dotenv_codegen::dotenv;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Body {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    stream: Option<bool>,
}

#[tokio::main]
async fn main() {
    const API_KEY: &str = dotenv!("API_KEY");
    let client = reqwest::Client::new();

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("date +%r")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    let mut messages: Vec<Message> = [
        Message {
            role: "system".to_string(),
            content: "You are a CLI based chatbot designed to help the user use their computer".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "You can run bash commands with `[run]: <command>` you'll get the console response to help you answer. Do this to gether up to date info. Improve your replies with info from the env, man pages, and the internet (curl)".to_string(),
        },
        Message {
            role: "assistant".to_string(),
            content: "Got it".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "What time is it".to_string(),
        },
        Message {
            role: "assistant".to_string(),
            content: "[run]: date +%T".to_string(),
        },
        Message {
            role: "system".to_string(),
            content: output.to_string(),
        },
    ].to_vec();
    
    loop {
        let input = inquire::Text::new(":").prompt();

        match input {
            Ok(input) => {
                messages.push(Message {
                    role: "user".to_string(),
                    content: input.clone(),
                });

                if input == "exit" {
                    break;
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        } 

        let body: Body = Body {
            model: "gpt-3.5-turbo".to_string(),
            messages: messages.clone(),
            max_tokens: None,
            temperature: None,
            top_p: None,
            stream: Some(false),
        };

        let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_string() + &API_KEY)
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await;

        match res {
            Ok(res) => {
                let body = res.text().await.unwrap();
                let body: serde_json::Value = serde_json::from_str(&body).unwrap();
                let response = body["choices"][0]["message"]["content"].as_str();

                match response {
                    Some(response) => {
                        println!("{}\n", response);
                        messages.push(Message {
                            role: "assistant".to_string(),
                            content: response.to_string(),
                        });

                        if response.contains("[run]:") {
                            let command = response.replace("[run]: ", "");
                            let output = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(command)
                                .output()
                                .expect("failed to execute process");
                            let output = String::from_utf8_lossy(&output.stdout);
                            println!("{}", output);
                            messages.push(Message {
                                role: "system".to_string(),
                                content: output.to_string(),
                            });
                        }
                    },
                    None => {
                        println!("Error: No response");
                    }
                }
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
