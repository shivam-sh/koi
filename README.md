# Koi

## What is Koi?

Koi is a simple tool built to let you use ChatGPT through the command line.
It adds the ability to let ChatGPT run commands on your computer in order to help you out, or to help you out with complicated tasks.

## Demo



## How do I use Koi?

Koi is a simple tool to use. You can use it by running the following command:
```bash
koi -a <OPENAI_API_KEY>
```

This will open up a prompt for you to start asking questions.

You can edit the configuration file to pre-fill the API key so you don't have to enter it every time, allowing you to just run `koi` to start using it.

## Installation

### From Source
To install Koi from source, you can run the following commands in your terminal:
```bash
cd koi
git clone https://github.com/shivam-sh/koi.git
cargo install --path .
```
You will need to have [Rust](https://www.rust-lang.org/tools/install) installed in order to install Koi from source.

### Download

- Download binaries for popular platforms from the [releases page](https://github.com/shivam-sh/koi/releases)
- Unzip the file with `tar -xvf <file>.tar.gz`
- Move the binary to a directory in your `PATH` environment variable

## Contributing

If you would like to contribute to Koi, you can do so by opening a pull request on the [GitHub repository](https://github.com/shivam-sh/koi).
