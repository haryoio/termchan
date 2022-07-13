use std::fmt::{Display, Error, Formatter};

pub struct Cookie {
    name:  String,
    value: String,
}

pub struct Cookies {
    cookies: Vec<Cookie>,
}

impl Cookies {
    pub fn new() -> Self {
        Cookies {
            cookies: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &str, value: &str) {
        self.cookies.push(Cookie {
            name:  name.to_string(),
            value: value.to_string(),
        });
    }

    pub fn keys(&self) -> Vec<&str> {
        self.cookies.iter().map(|c| c.name.as_str()).collect()
    }

    pub fn values(&self) -> Vec<&str> {
        self.cookies.iter().map(|c| c.value.as_str()).collect()
    }
}

impl Display for Cookies {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut cookies = Vec::new();
        for cookie in self.cookies.iter() {
            cookies.push(format!("{}={}", cookie.name, cookie.value));
        }
        write!(f, "{}", cookies.join("; "))
    }
}
