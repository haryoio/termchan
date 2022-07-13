use std::fmt::Display;

use scraper::Node;

#[derive(Debug)]
pub struct Message {
    pub image_url: Vec<String>,
    pub text:      Vec<Text>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for text in &self.text {
            write!(f, "{}", text)?;
        }
        Ok(())
    }
}

impl Message {
    pub fn new(message: &str) -> Self {
        parses_msg(message)
    }
}
impl Default for Message {
    fn default() -> Self {
        Message {
            image_url: Vec::new(),
            text:      Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Text {
    Plain(String),
    Link(String),
    Anchor(String, String),
    NewLine,
    Space,
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text::Plain(text) => write!(f, "{}", text),
            Text::Link(text) => write!(f, "{}", text),
            Text::Anchor(_, id) => write!(f, "{}", id),
            Text::NewLine => write!(f, "\n"),
            Text::Space => write!(f, " "),
        }
    }
}

pub fn parses_msg(s: &str) -> Message {
    let mut texts = vec![];
    let mut images = vec![];

    let fragment = scraper::Html::parse_fragment(s).tree;
    let mut values = fragment.values().peekable();

    loop {
        let current_node = values.next();
        if current_node.is_none() {
            break;
        }
        match current_node.unwrap() {
            Node::Text(text) => {
                let text = text.to_string();
                if text.len() <= 2 {
                    continue;
                }
                let mut first_camma = false;
                let text = if &text.chars().nth(0).unwrap() == &',' {
                    first_camma = true;
                    text[..text.len() - 1].to_string()
                } else {
                    text[1..text.len() - 1].to_string()
                };
                if text.len() == 0 {
                    continue;
                }

                if texts.last().is_some()
                    && (texts.last().unwrap() != &Text::NewLine
                        && texts.last().unwrap() != &Text::Space)
                {
                    if !first_camma {
                        texts.push(Text::Space);
                    }
                }
                texts.push(Text::Plain(text));
                texts.push(Text::Space);
            }
            Node::Element(element) => {
                match element.name() {
                    "a" => {
                        let class = element
                            .classes()
                            .find(|&class| class == "reply_link" || class == "image");

                        if texts.last().is_some()
                            && (texts.last().unwrap() != &Text::Space
                                && texts.last().unwrap() != &Text::NewLine)
                        {
                            texts.push(Text::Space);
                        }
                        match class {
                            Some("reply_link") => {
                                let url = element.attr("href").unwrap();
                                let text = values.next().unwrap().as_text().unwrap().to_string();
                                texts.push(Text::Anchor(url.to_string(), text));
                            }
                            Some("image") => {
                                let mut url = element.attr("href").unwrap().split("?");
                                url.next();
                                let url = url.next().unwrap().to_string();
                                texts.push(Text::Link(url.to_string()));
                                images.push(url.to_string());
                                values.next();
                            }
                            _ => unimplemented!(),
                        }
                        continue;
                    }
                    "br" => texts.push(Text::NewLine),
                    "body" | "html" | "span" => {}
                    _ => unimplemented!(),
                }
            }
            Node::Fragment => {}
            _ => {
                break;
            }
        }
    }
    Message {
        image_url: images,
        text:      texts,
    }
}

pub fn parse_msg(s: &str) -> Message {
    let spmsg = s.split("<");

    let mut msg = Vec::new();
    let mut image_url_list = Vec::new();
    for m in spmsg {
        // https://mi.5ch.net/test/read.cgi/news4vip/1657462844/l50
        if m.starts_with("br>") {
            msg.push(Text::NewLine);
        } else if m.starts_with("a href=") {
            let mut spurl = m.split("\"");
            spurl.next(); // <a href="
            let anchor_url = spurl.next().unwrap().to_string();
            let spurl = anchor_url.split("/");
            let anchor_text = &spurl.last().unwrap().to_string();
            msg.push(Text::Anchor(anchor_url, anchor_text.to_string()));
        } else if m.starts_with("a class=\"image\" href=") {
            let mut spurl = m.split("\"");
            spurl.next(); // a class=
            spurl.next(); // "image"
            spurl.next(); // "href="
            let image_url = spurl.next().unwrap().to_string();
            let mut image_url = image_url.split("?");
            image_url.next();
            let image_url = image_url.next().unwrap().to_string();
            msg.push(Text::Link(image_url.clone()));
            image_url_list.push(image_url.clone());
        } else {
            if m.contains("/a>") {
                msg.push(Text::Plain(m[3..].to_string()));
            } else {
                msg.push(Text::Plain(m.to_string()));
            }
        }
    }
    Message {
        image_url: image_url_list,
        text:      msg,
    }
}
