use std::time::{Duration, Instant};
use kukurust::client::MailClient;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mail_client = MailClient::new().await;
    println!("{} {}", mail_client.get_id(), mail_client.get_password());
    mail_client.new_temporary_mail().await;
    let mail_account = mail_client.new_temporary_mail().await;
    println!("{} {:?}", mail_account.get_mail_address(), start.elapsed());
    std::thread::sleep(Duration::from_secs(25));
    let mails = mail_client.get_mails(mail_account).await;
    println!("{}", mails.len());
}
