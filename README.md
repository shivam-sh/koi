# Koi

## What is Koi?

Koi is a simple tool built to let you use ChatGPT through the command line.
It adds the ability to let ChatGPT run commands on your computer in order to help you out, or to help you out with complicated tasks.

## Demo


https://user-images.githubusercontent.com/24887625/227695739-de3d96ff-94cc-42bf-80a9-a0d292d2fae9.mp4


## How do I use Koi?

Koi is a simple tool to use. You can use it by running the following command:
```bash
koi -a <OPENAI_API_KEY>
```

This will open up a prompt for you to start asking questions.

You can edit the configuration file to pre-fill the API key so you don't have to enter it every time, allowing you to just run `koi` to start using it.

## Installation

### Cargo
To install Koi using Cargo, you can run the following command in your terminal:
```bash
cargo install koios
```
You will need to have [Rust](https://www.rust-lang.org/tools/install) installed in order to install Koi from source.

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

On macOS, you may get an error saying that the binary is from an unidentified developer. To fix this, you can run the following command:
```bash
sudo xattr -r -d com.apple.quarantine <path to binary>
```
You can also just right click on the binary and click "Open" to open it for the first time.

## Contributing

If you would like to contribute to Koi, you can do so by opening a pull request on the [GitHub repository](https://github.com/shivam-sh/koi).
