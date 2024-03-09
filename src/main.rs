mod config_reader;
mod csv_reader;
mod email_sender;
mod template_processor;

use config_reader::read_config;
use csv_reader::read_csv;
use email_sender::EmailSender;
use std::error::Error;
use template_processor::{read_template, replace_placeholder};

fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config("config.toml")?;
    let email_sender = EmailSender::new(config.email, config.password);
    let records = read_csv(&config.csv_file_path)?;
    let template = read_template(&config.template_file_path)?;

    for record in records {
        if let Some(email) = record.get("email") {
            let email_body = replace_placeholder(&template, &record);

            // 使用 record 中的 email 欄位值作為收件人地址發送郵件
            email_sender.send_email(email, &config.title, &email_body)?;
        }
    }

    Ok(())
}
