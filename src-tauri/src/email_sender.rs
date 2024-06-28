use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub struct EmailSender {
    pub email: String,
    pub password: String,
}

impl EmailSender {
    pub fn new(email: String, password: String) -> Self {
        EmailSender { email, password }
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(self.email.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .unwrap();

        let creds = Credentials::new(self.email.clone(), self.password.clone());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent to {} successfully", to),
            Err(e) => panic!("Failed to send email to {}: {}", to, e),
        }
        Ok(())
    }
}
