use regex::Regex;

pub fn get_error(error: &str) -> Result<(), String> {
    let mut is_error = false;
    let mut message = "";

    let lines = error.split("\n");
    for line in lines {
        if line.contains("<title>") {
            if line.contains("ＥＲＲＯＲ！") {
                is_error = true;
            } else {
                return Ok(());
            }
        }
        let error_re =
            Regex::new(r##"<font size="+1" color="#FF0000"><b>ERROR: (.*)<br>"##).unwrap();
        if error_re.is_match(line) {
            message = error_re.captures(line).unwrap().get(1).unwrap().as_str();
        }
    }
    if is_error {
        return Err(message.to_string());
    } else {
        return Ok(());
    }
}
