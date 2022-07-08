fn parse_board_html(html: &str) {
}
fn parse_board_dat(dat: &str) {
}
// https://mi.5ch.net/news4vip/subject.txt
struct BoardSubject {
    name:  String,
    url:   String,
    id:    String,
    count: i32,
}

struct Board {
    name: String,
    url:  String,
}

impl Board {
    fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
    fn load(&self) -> anyhow::Result<Vec<Thread>> {
        let url = format!("{}{}", self.url, "subback.html");
        let html = Reciever::get(&url).await?.html();

        anyhow::Ok(normalize_board(html.as_str(), self.url.clone())?)
    }
}
