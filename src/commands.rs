use console::{style, Style};

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
            let output = if output.ends_with('\n') {
                output
            } else {
                output + "\n"
            };

            println!("{output}");

            responses.push(output);
        }
    }

    responses
}

pub fn run_checked(command: String) -> String {
    let error_style = Style::new().dim().red().bold();
    let command_not_run = error_style.apply_to("Command Not Run").to_string();

    let choices = vec!["Run Command", "Edit Command", "Cancel"];

    let choice = inquire::Select::new(
        &format!("{} {}\n", style("Command:").cyan().bold(), command),
        choices,
    )
    .without_help_message()
    .prompt();

    match choice {
        Ok(choice) => match choice {
            "Run Command" => {}
            "Edit Command" => {
                let new_command = inquire::Text::new(&style("Edit:").cyan().bold().to_string())
                    .with_initial_value(&command)
                    .prompt();

                match new_command {
                    Ok(new_command) => {
                        return run_checked(new_command);
                    }
                    Err(err) => {
                        eprintln!(
                            "{}",
                            error_style
                                .apply_to(format!("Inquire Error: {err}"))
                                .to_string()
                        );
                        return command_not_run;
                    }
                }
            }
            "Cancel" => {
                return error_style.apply_to("Command Run Cancelled\n").to_string();
            }
            _ => {
                eprintln!("{}", error_style.apply_to("Invalid Choice").to_string());
                return command_not_run;
            }
        },
        Err(err) => {
            eprintln!(
                "{}",
                error_style
                    .apply_to(format!("Inquire Error: {err}"))
                    .to_string()
            );
            return command_not_run;
        }
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(err) => error_style
            .apply_to(format!("Command Run Error: {err}"))
            .to_string(),
    }
}
