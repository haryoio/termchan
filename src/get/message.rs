use std::{fmt::Display, io::Write};

use reqwest::Url;
use serde::{Deserialize, Serialize};
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub images:  Vec<String>,
    pub text:    Vec<Text>,
    pub anchors: Vec<Text>,
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
        parse_msg(message)
    }
}
impl Default for Message {
    fn default() -> Self {
        Message {
            images:  Vec::new(),
            text:    Vec::new(),
            anchors: Vec::new(),
        }
    }
}

impl Message {
    pub fn json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Text {
    Plain(String),
    Link(String),
    Image(String),
    Anchors(Vec<Box<Text>>),
    AnchorRange(i32, i32),
    Anchor(i32),
    NewLine,
    Space,
    End,
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text::Plain(text) => write!(f, "{}", text),
            Text::Link(text) => write!(f, "{}", text),
            Text::AnchorRange(start, end) => write!(f, "{}-{}", start, end),
            Text::Anchor(num) => write!(f, "{}", num),
            Text::Anchors(texts) => {
                if texts.len() == 1 {
                    write!(f, "{}", texts[0])
                } else {
                    let mut anchors = vec![];
                    for text in texts {
                        anchors.push(format!("{}", text));
                    }
                    write!(f, "{}", anchors.join(","))
                }
            }
            Text::Image(text) => write!(f, "{}", text),
            Text::NewLine => write!(f, "\n"),
            Text::Space => write!(f, " "),
            Text::End => write!(f, ""),
        }
    }
}

pub fn parse_msg(s: &str) -> Message {
    let mut lexer = MessageLexers::<'_, Graphemes<'_>>::new(s);

    let mut tokens = vec![];
    loop {
        let token = lexer.next_token();
        if token == Token::End {
            break;
        }
        tokens.push(token);
    }

    MessageParser::new(tokens.iter()).parse()
}

struct MessageParser<'a, I>
where
    I: Iterator<Item = &'a Token> + Clone,
{
    input: I,
    token: Option<&'a Token>,
}

impl<'a, I> MessageParser<'a, I>
where
    I: Iterator<Item = &'a Token> + Clone,
{
    pub fn new(input: I) -> Self {
        let mut p = MessageParser { input, token: None };
        p.next();
        p
    }

    fn next(&mut self) {
        self.token = self.input.next();
    }

    pub fn parse(&mut self) -> Message {
        use Token::*;
        let mut images = vec![];
        let mut texts = vec![];
        let mut anchors = vec![];
        loop {
            match self.token {
                Some(Link(url)) => {
                    if is_image(url) {
                        texts.push(Text::Image(url.to_string()));
                        images.push(url.clone());
                    } else {
                        texts.push(Text::Link(url.to_string()));
                    }
                    self.next();
                }
                Some(Char(c)) => {
                    let mut text = String::new();
                    text.push(*c);
                    loop {
                        self.next();
                        match self.token {
                            Some(Char(ch)) => {
                                text.push(*ch);
                            }
                            _ => break,
                        }
                    }

                    texts.push(Text::Plain(text));
                }
                Some(NewLine) => {
                    texts.push(Text::NewLine);
                    self.next();
                }
                Some(Space) => {
                    texts.push(Text::Space);
                    self.next();
                }
                Some(End) => {
                    texts.push(Text::End);
                    self.next();
                }
                Some(Gt) => {
                    self.next();
                    if !matches!(self.token, Some(Gt)) {
                        texts.push(Text::Plain(">".to_string()));
                        continue;
                    }
                    self.next();
                    let mut anchors_tmp = vec![];
                    loop {
                        if matches!(self.token, Some(Number(_))) {
                            let mut n = 0;
                            if let Some(Number(num)) = self.token {
                                n = *num
                            }
                            self.next();

                            if let Some(Hyphen) = self.token {
                                self.next();

                                if let Some(Number(m)) = self.token {
                                    anchors_tmp
                                        .push(Box::new(Text::AnchorRange(n as i32, *m as i32)));
                                    self.next();
                                } else {
                                    anchors_tmp.push(Box::new(Text::Anchor(n as i32)));
                                    break;
                                }
                            }

                            if let Some(Comma) = self.token {
                                anchors_tmp.push(Box::new(Text::Anchor(n as i32)));
                                self.next();
                                continue;
                            }
                        }
                        break;
                    }
                    texts.push(Text::Anchors(anchors_tmp.clone()));
                    anchors.push(Text::Anchors(anchors_tmp.clone()))
                }
                Some(Number(n)) => {
                    texts.push(Text::Plain(n.to_string()));
                    self.next();
                }
                Some(Hyphen) => {
                    texts.push(Text::Plain("-".to_string()));
                    self.next();
                }
                Some(Comma) => {
                    texts.push(Text::Plain(",".to_string()));
                    self.next();
                }
                Some(Div(class, body)) => {
                    let body = body.iter().map(|b| b.as_ref()).collect::<Vec<&Token>>();
                    let mut inner = MessageParser::new(body.into_iter()).parse();
                    texts.append(&mut inner.text);
                    self.next();
                }
                Some(DivEnd) => {
                    self.next();
                    break;
                }
                None => break,
                _ => panic!("unexpected token :{:?}", self.token.unwrap()),
            }
        }
        Message {
            images,
            text: texts,
            anchors,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Gt,
    Comma,
    Quote,
    Space,
    Hyphen,
    NewLine,
    End,
    Char(char),
    Str(String),
    Number(i64),
    Link(String),
    Error(String),
    /// Vec<Class>, InnerText
    Div(Vec<String>, Vec<Box<Token>>),
    DivEnd,
}

#[derive(Clone, Debug)]
pub struct MessageLexers<'a, I: Iterator<Item = &'a str> + Clone> {
    pub input:  I,
    pub ch:     char,
    pub offset: usize,
}

impl<'a, I> MessageLexers<'a, I>
where
    I: Iterator<Item = &'a str> + Clone,
{
    pub fn new<'b>(input: &'b str) -> MessageLexers<Graphemes<'b>> {
        let iter = input.graphemes(true);

        MessageLexers {
            input:  iter,
            ch:     ' ',
            offset: 0,
        }
    }

    /// 次のトークンを取得する。
    pub fn next_token(&mut self) -> Token {
        self.read_char();
        match self.ch {
            '<' => self.read_tag(),
            '&' => self.read_escaped_char(),
            '-' => Token::Hyphen,
            ',' => Token::Comma,
            ' ' => Token::Space,
            'h' => {
                let url = self.read_url();
                if url.is_empty() {
                    Token::Char('h')
                } else {
                    Token::Link(url)
                }
            }
            '0'..='9' => self.read_number(),
            '\n' => Token::NewLine,
            '\0' => Token::End,
            _ => Token::Char(self.ch),
        }
    }

    /// 連続した数字を全て取得し、i64でパースする。
    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        num.push(self.ch);
        let iter = self.input.clone();
        let mut iter = iter.clone().peekable();
        let mut count = 0;

        loop {
            match iter.peek().map(|s| s.chars().next().unwrap()) {
                Some(ch) => {
                    if is_digit(ch) {
                        num.push(ch);
                        iter.next();
                    } else {
                        break;
                    }
                }
                None => break,
            }
            count += 1;
        }

        while count > 0 {
            self.read_char();
            count -= 1;
        }

        Token::Number(num.parse::<i64>().unwrap())
    }

    /// hから始まり、半角スペースが出るまでを取得する。
    fn read_url(&mut self) -> String {
        let mut text = String::new();
        text.push('h');
        let iter = self.input.clone();
        let mut iter = iter.clone().peekable();
        let mut count = 0;
        loop {
            match iter.peek().map(|s| s.chars().next().unwrap()) {
                Some(ch) => {
                    if matches!(ch, ' ' | '\n' | '<' | '\0') {
                        break;
                    }
                    text.push(ch);
                    iter.next();
                }
                None => break,
            }
            count += 1;
        }

        let url = Url::parse(&text);
        if url.is_ok() {
            while count > 0 {
                self.read_char();
                count -= 1;
            }
            text
        } else {
            String::new()
        }
    }

    fn read_tag(&mut self) -> Token {
        let mut text = String::new();
        let mut classes = vec![];
        text.push(self.ch);
        while self.ch != '>' {
            self.read_char();
            text.push(self.ch);
            if self.ch == 'c' {
                // class= を読み飛ばす。
                while self.ch != '\"' {
                    self.read_char();
                }
                self.read_char();
                let mut class = String::new();
                while self.ch != '\"' {
                    if self.ch == ' ' {
                        self.read_char();
                        classes.push(class);
                        class = String::new();
                    }
                    class.push(self.ch);
                    self.read_char();
                }
                classes.push(class);
            }
        }

        if matches!(text.as_str(), "<br>") {
            return Token::NewLine;
        }
        if text.starts_with("</") {
            return Token::DivEnd;
        }

        let mut tokens = vec![];
        loop {
            let token = self.next_token();
            match token {
                Token::End => break,
                _ => tokens.push(Box::new(token)),
            }
        }
        Token::Div(classes, tokens)
    }

    /// エスケープされている特定の文字列を取得する。
    /// &から始まり、;で終わる。
    fn read_escaped_char(&mut self) -> Token {
        let mut text = String::new();
        text.push(self.ch);

        self.read_char();
        if !(self.ch.is_alphanumeric() && self.ch == '#') {
            return Token::Char(self.ch);
        }
        text.push(self.ch);

        while self.ch != ';' {
            self.read_char();
            text.push(self.ch);
        }

        if text.starts_with("&#") {
            let num = text[2..text.len() - 1].to_string().parse::<u32>().unwrap();
            let c = char::from_u32(num);
            if let Some(c) = c {
                return Token::Char(c);
            }
        }

        match text.as_str() {
            "&amp;" => Token::Char('&'),
            "&quot;" => Token::Char('"'),
            "&lt;" => Token::Char('<'),
            "&gt;" => Token::Gt,
            _ => Token::Str(text),
        }
    }

    pub fn read_char(&mut self) {
        self.ch = match self.input.next() {
            Some(ch) => ch,
            None => "\0",
        }
        .chars()
        .next()
        .unwrap();
        self.offset = 0;
    }

    pub fn peek(&mut self) -> Option<char> {
        self.offset += 1;
        let c = self
            .input
            .nth(self.offset)
            .map(|s| s.chars().next().unwrap());
        c
    }
}

fn is_digit(ch: char) -> bool {
    matches!(ch, '0'..='9')
}

fn is_image(url: &str) -> bool {
    let url = url.split('.');
    let ext = url.last().unwrap();
    match ext {
        "png" | "jpg" | "jpeg" | "gif" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_lex_message<'a>() {
        let texts = vec![
            "              &gt;&gt;1   &gt;&gt;1,2    &gt;&gt;11-200   asdcasdc",
            "&gt;asdfadfadfadfss   asdf &gt;&gt;100,200-300,1111 ",
            "http://hello.com/hellom ",
            "https://image.com/image.png <br>",
            "<br>aaaaa<br>    ",
            "全角",
        ];

        for input in texts {
            let mut parser = MessageLexers::<'a, Graphemes<'_>>::new(input);

            loop {
                let token = parser.next_token();
                match token {
                    Token::End => break,
                    _ => println!("{:?}", token),
                }
            }
        }
    }

    #[test]
    fn test_lex_escaped_chars<'a>() {
        let tests = vec![
            ("&lt;", Token::Char('<')),
            ("&gt;", Token::Gt),
            ("&amp;", Token::Char('&')),
            ("&quot;", Token::Char('\"')),
            ("&#12785;", Token::Char('ㇱ')),
        ];
        for (input, output) in tests {
            let mut parser = MessageLexers::<'a, Graphemes<'_>>::new(input);
            let token = parser.next_token();
            assert_eq!(token, output);
        }
    }

    #[test]
    fn test_lex_tag<'a>() {
        use Token::*;
        let tests = vec![
            ("<br>", vec![NewLine]),
            (
                "<br> <br> <br>",
                vec![NewLine, Space, NewLine, Space, NewLine],
            ),
            (
                "<div class=\"tanzaku tblue\">M</div>",
                vec![Div(
                    vec!["tanzaku".to_string(), "tblue".to_string()],
                    vec![Box::new(Char('M')), Box::new(DivEnd)],
                )],
            ),
        ];
        for (input, output) in tests {
            let mut parser = MessageLexers::<'a, Graphemes<'_>>::new(input);
            for t in output {
                let token = parser.next_token();
                assert_eq!(token, t);
            }
        }
    }

    #[test]
    fn test_lex_url<'a>() {
        use Token::*;
        let tests = vec![
            (
                "http://hello.com/hellom",
                vec![Link("http://hello.com/hellom".to_string())],
            ),
            (
                "https://image.com/image.png",
                vec![Link("https://image.com/image.png".to_string())],
            ),
        ];
        for (input, output) in tests {
            let mut parser = MessageLexers::<'a, Graphemes<'_>>::new(input);
            for t in output {
                let token = parser.next_token();
                assert_eq!(token, t);
            }
        }
    }

    #[test]
    fn test_parse_string() {
        let tests = vec![
            "hello",
            "&gt;asdfadfadfadfss   asdf &gt;&gt;100-200,300,111-1000 <br> <br>",
            "&#12785;",
        ];

        for input in tests {
            let message = parse_msg(input);
            println!("{}", message);
        }
    }
}
