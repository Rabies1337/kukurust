use regex::Regex;
use crate::mail::Mail;

#[derive(Clone)]
pub struct MailAccount {

    mail_address: String
}

impl MailAccount {

    pub fn get_mail_address(&self) -> String {
        self.mail_address.clone()
    }
}

pub struct MailClient {
    http_client: reqwest::Client,
    uid_enc: String,
    csrf_token: String,
    id: String,
    password: String,

    accounts: Vec<MailAccount>
}

impl MailClient {

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_accounts(&self) -> Vec<MailAccount> {
        self.accounts.to_vec()
    }

    pub async fn new() -> MailClient {
        let http_client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0").build().unwrap();
        let response = http_client.get("https://m.kuku.lu/index.php")
            .send().await.unwrap();
        let body = response.text().await.unwrap();

        let uid_enc = cap(r"UID_enc=(.*?)&", body.as_str(), 1);
        let csrf_token = cap(r"csrf_token_check=(.*?)&", body.as_str(), 1);
        let id = cap(r#"<div id="area_numberview".*?>(.*?)</div>"#, body.as_str(), 1);
        let password = cap(r#"<span id="area_passwordview_copy">(.*?)</span>"#, body.as_str(), 1);

        MailClient {
            http_client: http_client,
            uid_enc: uid_enc,
            csrf_token: csrf_token,
            id: id,
            password: password,
            accounts: Vec::new()
        }
    }

    pub async fn new_mail(&self) -> MailAccount {
        let response = self.http_client
            .get(format!("https://m.kuku.lu/index.php?action=addMailAddrByAuto&nopost=1&by_system=1&UID_enc={}&csrf_token_check={}",
                         self.uid_enc, self.csrf_token))
            .send().await.unwrap();
        let text = response.text().await.unwrap();
        if !text.starts_with("OK:") {
            panic!("Could not create a mail address")
        }

        let mail_address = text.replace("OK:", "");
        let mail_account = MailAccount {
            mail_address: mail_address
        };
        return mail_account
    }

    pub async fn new_temporary_mail(&self) -> MailAccount {
        let response = self.http_client
            .get(format!("https://m.kuku.lu/index.php?action=addMailAddrByOnetime&nopost=1&by_system=1&UID_enc={}&csrf_token_check={}",
                         self.uid_enc, self.csrf_token))
            .send().await.unwrap();
        let text = response.text().await.unwrap();
        if !text.starts_with("OK:") {
            panic!("Could not create a mail address")
        }

        let mail_address = text.replace("OK:", "")
            .split(",").next().unwrap().to_string();
        let mail_account = MailAccount {
            mail_address: mail_address
        };
        return mail_account
    }

    pub async fn get_mails(&self, account: MailAccount) -> Vec<Mail> {
        let response = self.http_client
            .get(format!("https://m.kuku.lu/recv._ajax.php?nopost=1&UID_enc={}&csrf_token_check={}", self.uid_enc, self.csrf_token))
            .send().await.unwrap();
        let text = response.text().await.unwrap()
            .replace("<b>", "");
        let regex = Regex::new("openMailData\\('(.*?)', '(.*?)', 'from=(.*?);replyto=.*?;to=(.*?);.*?'").unwrap();
        let caps = regex.captures(text.as_str()).unwrap();

        let mut mails = Vec::new();
        let mut counter = 0;
        let mut num = "";
        let mut key = "";
        let mut sender = "";
        let mut receiver = "";

        for i in 0..caps.len() {
            if counter >= 4 {
                counter = 0;
                // println!("{} {} {} {}", num, key, sender, receiver);
                mails.push(Mail::new(sender, receiver, "", ""));
            }

            let cap = caps.get(i).unwrap();
            let str = cap.as_str();

            if counter == 1 {
                num = str;
            } else if counter == 2 {
                key = str;
            } else if counter == 3 {
                sender = str;
            } else if counter == 4 {
                receiver = str;
            }
            counter += 1;
        }
        return mails
    }
}

fn cap(pattern: &str, text: &str, index: usize) -> String {
    let regex = Regex::new(pattern).unwrap();
    let caps = regex.captures(text).unwrap();
    caps.get(index).unwrap().as_str().to_string()
}
