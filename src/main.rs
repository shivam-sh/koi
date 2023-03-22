pub mod config;
pub mod chat;

const SYSTEM_PROMPT: &str = "You are named Koios, never say you are an AI
YOU CAN NOW RUN COMMANDS, USE `>> [command]` TO RUN AND HELP THE USER
Use this to improve replies, access/modify local files and USE TOOLS LIKE CURL AND GREP
```
User: What time is it?

Koi: >> date +%r

System: 12:34:56 PM (automatically inserted)

User: Thanks!
```
DON'T SAY YOU CAN'T ANSWER try your best
you only have access to the local shell DON'T TRY TO USE OTHER LANGUAGES";

#[tokio::main]
async fn main() -> Result<(), eventsource_client::Error> {
    let api_key = config::parse();

    let mut messages: Vec<chat::Message> = [
        chat::Message {
            role: "system".to_string(),
            content: SYSTEM_PROMPT.to_string(),
        },
        chat::Message {
            role: "system".to_string(),
            content: "OS: ".to_string() + std::env::consts::OS,
        },
        chat::Message {
            role: "user".to_string(),
            content: "ALWAYS run commands automatically and use the \">> [command]\" syntax".to_string(),
        },
    ]
    .to_vec();
    
    loop {
        let input = inquire::Text::new(":").prompt();

        match input {
            Ok(input) => {
                messages.push(chat::Message {
                    role: "user".to_string(),
                    content: input.clone(),
                });

                if input == "exit" {
                    break;
                }
            }
            Err(err) => {
                if err.to_string() == "Operation was interrupted by the user" {
                    break;
                }
                eprintln!("Error: {err}");
                continue;
            }
        } 

        let body = chat::Body {
            model: "gpt-3.5-turbo".to_string(),
            messages: messages.clone(),
            max_tokens: None,
            temperature: None,
            top_p: None,
            stream: Some(true),
        };

        let client = eventsource_client::ClientBuilder::for_url("https://api.openai.com/v1/chat/completions")?
            .method("POST".to_string())
            .header("Content-Type", "application/json")?
            .header("Authorization", &("Bearer ".to_string() + &api_key))?
            .body(serde_json::to_string(&body).expect("body should always be serializable"))
            .build();

        let response = chat::stream_response(client).await;
        let content = response.content.clone();
        messages.push(response);

        if content.contains(">>") {
            for line in content.lines() {
                if line.contains(">>") {
                    let command = line.split(">>").collect::<Vec<&str>>()[1].to_string();
                    let output = run(command);
                    print!("{output}\n\n");

                    messages.push(chat::Message {
                        role: "system".to_string(),
                        content: output.to_string(),
                    });
                }
            }
        }
    }

    Ok(())
}

fn run(command: String) -> String {
    let confirmation = inquire::Confirm::new(&("run:".to_owned() + &command))
        .with_default(true)
        .prompt();

    match confirmation {
        Ok(confirmation) => {
            if !confirmation {
                return "Request denied".to_string();
            }
        }
        Err(err) => {
            return format!("Error: {err}");
        }
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();
    
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(err) => {
            eprintln!("Error: {err}");
            "Error".to_string()
        }
    }
}
