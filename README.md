# Email Sender

[![CI](https://github.com/YJack0000/email-sender/actions/workflows/release.yml/badge.svg)](https://github.com/YJack0000/email-sender/actions/workflows/release.yml)
[![License](https://img.shields.io/github/license/YJack0000/email-sender)](https://github.com/YJack0000/email-sender/blob/main/LICENSE)

The Rust Email Sender project is an efficient and flexible automated email sending tool, designed to simplify the process of sending personalized emails in bulk. Whether for marketing campaigns, event notifications, or customer communications, the Rust Email Sender meets your needs with ease.

## Features

- Configuration-Driven: Easily set SMTP server details, including email address and password, through a TOML configuration file. Dynamic configuration eliminates the need for hardcoding.
- CSV Data Handling: Automatically reads CSV files, mapping data to email templates for dynamic field replacement, allowing each email to be personalized according to recipient information.
- Template Support: Utilize custom templates to generate email content, offering flexible content design tailored to your targets and scenarios.
- Cross-Platform Compilation: Supports compilation across various platforms, including macOS, Windows, and Linux, facilitated by GitHub Actions for automated testing and building.
- Simplified API: Offers a straightforward API, making the email sending process intuitively clear.
- Secure and Reliable: Employs standard SMTP protocols and secure authentication mechanisms to ensure the security of email transmissions.

## Getting Started

The program will read the configuration from a file named `config.toml` in the same directory as the executable. The configuration file should contain the following fields:

`config.toml`

```toml
email = "user@example.com"
password = "secret"
title = "The title of the email"
csv_file_path = "data.csv"
template_file_path = "template.txt"
```

You can generate you gmail password [here](https://myaccount.google.com/apppasswords)

Then, prepare your CSV data file and email template. The CSV file should contain a header row with the field names, and the email template should be a text file with the fields enclosed in double curly braces, like so:

`template.txt`

```txt
Hello {name}! I want to tell you about our new product, {product}.
```

`user.csv`

```csv
name,email,product
John Doe,john@gmail.com,Widget
```

The resulting email will be:

```txt
Hello John Doe! I want to tell you about our new product, Widget.
```

### Development Environment

1. Clone the repository to your local machine:

```bash
git clone https://github.com/YJack0000/email-sender.git
```

2. Create a config.toml in the project root directory and fill in your SMTP server and authentication details.
3. Prepare your CSV data file and email template.
4. Compile and run the project:

```bash
cargo build --release
cargo run
```

### Usage

1. Download the latest release from the [Releases](https://github.com/YJack0000/email-sender/releases)
2. Create a config.toml in the same directory as the executable and fill in your SMTP server and authentication details.
3. Prepare your CSV data file and email template.
4. Run the executable: `email-sender` or `email-sender.exe`

## How to Contribute

Issues and suggestions can be submitted via GitHub Issues, and Pull Requests are equally welcome. If you have any ideas for improving the code or adding new features, don't hesitate to get involved!
