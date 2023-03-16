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

const SYSTEM_PROMPT: &str = 
"You are named Koios, never say you are an AI
YOU CAN NOW RUN COMMANDS, USE `>> [command]` TO RUN AND HELP THE USER
Use this to improve replies, access/modify local files and USE TOOLS LIKE CURL AND GREP
```
User: What time is it?

Koi: >> date +%r

System: 12:34:56 PM (command ran)

User: Thanks!
```
ALWAYS RUN COMMANDS AUTOMATICALLY AND USE THE FORMAT ABOVE
DON'T SAY YOU CAN'T ANSWER try your best
you only have access to the local shell DON'T TRY TO USE OTHER LANGUAGES";

#[tokio::main]
async fn main() {
    const API_KEY: &str = dotenv!("API_KEY");
    let client = reqwest::Client::new();

    let mut messages: Vec<Message> = [
        Message {
            role: "system".to_string(),
            content: SYSTEM_PROMPT.to_string(),
        },
        Message {
            role: "system".to_string(),
            content: "OS: ".to_string() + &std::env::consts::OS.to_string(),
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
                if err.to_string() == "Operation was interrupted by the user" {
                    break;
                } else {
                    println!("Error: {}", err);
                }
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

                        if response.contains(">>") {
                            for line in response.lines() {
                                if line.contains(">>") {
                                    let command = line.split(">>").collect::<Vec<&str>>()[1].to_string();
                                    let output = run(command);
                                    println!("{}", output);
                                    messages.push(Message {
                                        role: "system".to_string(),
                                        content: output.to_string(),
                                    });
                                }
                            }
                        }
                    },
                    None => {
                        println!("Error: Invalid response");
                    }
                }
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}

fn run(command: String) -> String {
    let confirmation = inquire::Confirm::new(&("run:".to_owned() + &command))
        .with_default(true)
        .prompt();

    match confirmation {
        Ok(confirmation) => {
            if !confirmation {
                return "Cancelled".to_string();
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            return "Error".to_string();
        }
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    output.to_string()
}
