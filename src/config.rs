use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_key: "OpenAI API Key".into(),
        }
    }
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

    let config: Result<Config, confy::ConfyError> = confy::load("koi", "config");
    if let Ok(config) = config {
        if config.api_key != "OpenAI API Key" {
            return config.api_key;
        }
    }

    eprintln!("No OpenAI API Key found in config, env (OPENAI_API_KEY), or flags. Please provide one to run.");
    let api_key = inquire::Password::new("OpenAI API Key:").prompt();

    match api_key {
        Ok(api_key) => api_key,
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}
