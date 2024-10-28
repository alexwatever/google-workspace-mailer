use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "";
    const PASSWORD: &str = "";
    const RECIPIENT: &str = "";
    const SUBJECT: &str = "Test email";
    const BODY: &str = "Hello, this is a test email.";

    // Create the email
    let email = Message::builder()
        .from(EMAIL.parse()?)
        .to(RECIPIENT.parse()?)
        .subject(SUBJECT)
        .body(BODY.to_string())?;

    // Use your email and app password or regular password if less secure apps are enabled
    let creds = Credentials::new(EMAIL.to_string(), PASSWORD.to_string());

    // Set up the SMTP transport
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email)?;

    println!("Email sent successfully!");

    Ok(())
}
