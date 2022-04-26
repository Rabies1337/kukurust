
pub struct Mail {
    from: String,
    to: String,
    subject: String,
    body: String
}

impl Mail {

    pub fn new(from: &str, to: &str, subject: &str, body: &str) -> Mail {
        Mail { from: from.to_string(), to: to.to_string(), subject: subject.to_string(), body: body.to_string() }
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn to(&self) -> String {
        self.to.clone()
    }

    pub fn subject(&self) -> String {
        self.subject.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}
