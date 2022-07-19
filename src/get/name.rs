use std::fmt::Display;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Name {
    pub mail: Option<String>,
    pub name: String,
    pub cote: Option<String>,
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        Ok(())
    }
}

impl Name {
    pub fn new(name: &str) -> Self {
        parse_name(name)
    }
}
/// Parse name from string.
/// 読みにくいので改修する必要あり。

#[allow(unused_assignments)]
fn parse_name(name: &str) -> Name {
    let mut mail = None;
    let mut cote = None;
    let mut n = "".to_string();
    if name.starts_with("<a href=\"mailto:") {
        let mut sp = name.split("<a href=\"mailto:").nth(1).unwrap().split("\">");
        mail = Some(sp.next().unwrap().to_string());
        let nn = sp.next().unwrap().to_string();
        let nn = nn[..nn.len() - 4].to_string();
        if nn.contains("</b>") {
            let re = Regex::new(r"</b>(.*) <b>").unwrap();
            let caps = re.captures(nn.as_str());
            match caps {
                Some(caps) => {
                    cote = Some(caps.get(1).unwrap().as_str().to_string());
                    n = name.replace(&caps.get(0).unwrap().as_str(), "");
                }
                None => {
                    n = name.to_string();
                }
            }
        } else {
            n = nn;
        }
    } else {
        mail = None;
        if name.contains("<") {
            let re = Regex::new(r"</b>(.*) <b>").unwrap();
            let caps = re.captures(name);
            match caps {
                Some(caps) => {
                    cote = Some(caps.get(1).unwrap().as_str().to_string());
                    n = name.replace(&caps.get(0).unwrap().as_str(), "");
                }
                None => {
                    n = name.to_string();
                }
            }
        } else {
            n = name.to_string();
            cote = None;
        }
    };
    Name {
        mail,
        name: n,
        cote,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_name() {
        let names = vec![
            r#"name"#,
            r#"name </b>◆coteihandler <b>"#,
            r#"<a href="mailto:info@example.co.com"></b>◆coteihandler <b></a>"#,
            r#"<a href="mailto:info@example.co.com">なまえ </b>◆coteihandler <b></a>"#,
            r#"<a href="mailto:sage">名前</a>"#,
            r#"<a href="mailto:sage">なまえ</a>"#,
        ];
        for name in names {
            let name = parse_name(name);
            println!("{:?}", name);
        }
    }
}
