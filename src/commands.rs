#[must_use]
pub fn parse(input: &str) -> Vec<String> {
    let mut responses: Vec<String> = Vec::new();

    if input.contains("```") {
        let command_blocks = input
            .split("```")
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, block)| block)
            .collect::<Vec<&str>>();

        for block in command_blocks {
            let block = block
                .lines()
                .skip(1)
                .take_while(|line| line != &"")
                .collect::<Vec<&str>>()
                .join("\n");

            let output = run(block.to_string());
            println!("{output}");
            responses.push(output);
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
