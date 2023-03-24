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

            let output = run_checked(block.to_string());
            println!("{output}\n");
            responses.push(output);
        }
    }

    responses
}

pub fn run_checked(command: String) -> String {
    let choices = vec![
        "Run Command",
        "Edit Command",
        "Cancel"
        ];

    let choice = inquire::Select::new(
        &("Command: ".to_owned() + &command + "\n"),
        choices,
    )
    .without_help_message()
    .prompt();

    match choice {
        Ok(choice) => {
            match choice {
                "Run Command" => {}
                "Edit Command" => {
                    let new_command = inquire::Text::new("Edit Command:")
                    .with_initial_value(&command)
                    .prompt();

                    match new_command {
                        Ok(new_command) => {
                            return run_checked(new_command);
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            return "command not run".to_string();
                        }
                    }
                }
                "Cancel" => {
                    return "command not run".to_string();
                }
                _ => {
                    eprintln!("Error: Invalid choice");
                    return "command not run".to_string();
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return "Command run cancelled".to_string();
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
