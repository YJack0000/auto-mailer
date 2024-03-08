# Rust Email Sender

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

## How to Contribute

Issues and suggestions can be submitted via GitHub Issues, and Pull Requests are equally welcome. If you have any ideas for improving the code or adding new features, don't hesitate to get involved!
