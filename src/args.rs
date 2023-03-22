use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    api_key: Option<String>,
}

#[must_use]
pub fn parse() -> String {
    let args = Args::parse();

    if let Some(api_key) = args.api_key {
        return api_key;
    }

    if let Some(api_key) = option_env!("OPENAI_API_KEY") {
        return api_key.to_string();
    }

    eprintln!("No API Key found in flags or env (OPENAI_API_KEY), please provide one now.");
    let api_key = inquire::Password::new("OpenAI API Key:").prompt();

    match api_key {
        Ok(api_key) => api_key,
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}
