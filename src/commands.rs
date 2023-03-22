#[must_use]
pub fn parse(input: &str) -> Vec<String> {
    let mut responses: Vec<String> = Vec::new();

    if input.contains(">>") {
        for line in input.lines() {
            if line.contains(">>") {
                let command = line.split(">>").collect::<Vec<&str>>()[1].to_string();
                let output = run(command);
                print!("{output}\n\n");

                responses.push(output.to_string());
            }
        }
    }

    responses
}

pub fn run(command: String) -> String {
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
